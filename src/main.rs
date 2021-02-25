use reqwest;
use serde_json::Value;
use tokio::fs;
use structopt::StructOpt;

use std::path::PathBuf;

use std::io;

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

async fn get_weather(api: &str, loc: &str) -> String {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        loc, api
    );
    let response = reqwest::get(&url).await.unwrap().text().await.unwrap();

    //converting to json so it can be printed
    let v: Value = serde_json::from_str(&response).unwrap();

    format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        v["dt"],
        v["name"],
        v["sys"]["country"],
        v["coord"]["lon"],
        v["coord"]["lat"],
        v["weather"][0]["main"],
        v["weather"][0]["description"],
        v["weather"][0]["icon"],
        v["sys"]["sunrise"],
        v["sys"]["sunset"],
        v["clouds"]["all"],
        v["wind"]["speed"],
        v["wind"]["deg"],
        v["visibility"],
        v["rain"]["1h"],
        v["rain"]["3h"],
        v["snow"]["1h"],
        v["snow"]["3h"],
        v["main"]["temp_min"],
        v["main"]["temp_max"],
        v["main"]["temp"],
        v["main"]["feels_like"],
        v["main"]["humidity"],
        v["main"]["pressure"],
    )
}

async fn get_city(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //reading in loc and apikey
    let opt = Opt::from_args();
    let api = fs::read_to_string(opt.apikey_file).await?;

    //trimming
    api.trim_matches(char::is_control).to_string();

    let locs = if opt.isp_loc {
        vec![get_city("http://ip-api.com/line/?fields=city").await]
    } else {
        fs::read_to_string(opt.locations_file).await?
            .trim_matches(char::is_control)
            .to_string()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    };

    for loc in locs {
        println!("{}", get_weather(&api, &loc).await);
    }
    Ok(())
}
