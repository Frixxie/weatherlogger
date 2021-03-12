use reqwest::get;
use serde_json::Value;
use structopt::StructOpt;
use tokio::fs;
mod weather;

use std::io;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "weatherlogger",
    about = "Logs the weather from https://openweathermap.com"
)]
struct Opt {
    ///Use isp location
    #[structopt(short, long)]
    isp_loc: bool,

    ///Apikey filename
    #[structopt(short, long, default_value = "./apikey")]
    apikey_file: PathBuf,

    ///Locations filename
    #[structopt(short, long, default_value = "./locations")]
    locations_file: PathBuf,
}

async fn get_weather(api: &str, loc: &str) -> weather::Weather {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        loc, api
    );
    let response = get(&url).await.unwrap().text().await.unwrap();

    //converting to json so it can be printed
    let v: Value = serde_json::from_str(&response).unwrap();

    weather::Weather::new(
        v["dt"].as_u64().unwrap_or(0) as u32,
        v["name"].to_string(),
        v["sys"]["country"].to_string(),
        v["coord"]["lon"].as_f64().unwrap_or(0.0) as f32,
        v["coord"]["lat"].as_f64().unwrap_or(0.0) as f32,
        v["weather"][0]["main"].to_string(),
        v["weather"][0]["description"].to_string(),
        v["weather"][0]["icon"].to_string(),
        v["sys"]["sunrise"].as_u64().unwrap_or(0) as u32,
        v["sys"]["sunset"].as_u64().unwrap_or(0) as u32,
        v["clouds"]["all"].as_u64().unwrap_or(0) as u32,
        v["wind"]["speed"].as_f64().unwrap_or(0.0) as f32,
        v["wind"]["deg"].as_i64().unwrap_or(0) as i32,
        v["visibility"].as_i64().unwrap_or(0) as i32,
        v["rain"]["1h"].as_f64().unwrap_or(0.0) as f32,
        v["rain"]["3h"].as_f64().unwrap_or(0.0) as f32,
        v["snow"]["1h"].as_f64().unwrap_or(0.0) as f32,
        v["snow"]["3h"].as_f64().unwrap_or(0.0) as f32,
        v["main"]["temp_min"].as_f64().unwrap_or(0.0) as f32,
        v["main"]["temp_max"].as_f64().unwrap_or(0.0) as f32,
        v["main"]["temp"].as_f64().unwrap_or(0.0) as f32,
        v["main"]["feels_like"].as_f64().unwrap_or(0.0) as f32,
        v["main"]["humidity"].as_u64().unwrap_or(0) as u32,
        v["main"]["pressure"].as_u64().unwrap_or(0) as u32,
    )
}

async fn get_city(url: &str) -> String {
    get(url).await.unwrap().text().await.unwrap()
}

async fn get_locs(isp_loc: bool, loc_f: PathBuf) -> Vec<String> {
    if isp_loc {
        vec![get_city("http://ip-api.com/line/?fields=city").await]
    } else {
        fs::read_to_string(loc_f)
            .await
            .unwrap()
            .trim_matches(char::is_control)
            .to_string()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //reading in loc and apikey
    let opt = Opt::from_args();

    //reading in api and locs
    let (api_res, locs) = tokio::join!(
        fs::read_to_string(opt.apikey_file),
        get_locs(opt.isp_loc, opt.locations_file)
    );

    //trimming
    let api = api_res.unwrap().trim_matches(char::is_control).to_string();

    //vector for containing the join_handles spawned from the tokio threads
    let mut futures = Vec::new();
    for loc in locs {
        //needed for move
        let api_clone = api.clone();
        futures.push(tokio::spawn(
            async move { get_weather(&api_clone, &loc).await },
        ));
    }

    let mut reses = Vec::<weather::Weather>::new();
    //getting the results or something
    for future in futures {
        reses.push(future.await?);
    }

    for res in &reses {
        println!("{}", res);
    }

    Ok(())
}
