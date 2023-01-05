extern crate serde;
mod weather;

use futures::future::try_join_all;
use reqwest::get;
use serde::Deserialize;
use serde_json::Value;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::fs;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "weatherlogger",
    about = "Logs the weather from https://openweathermap.com"
)]
struct Opt {
    ///Use isp location
    #[structopt(short, long)]
    isp_loc: bool,

    ///Config file
    #[structopt(short, long, default_value = "./config.json")]
    config_file: PathBuf,
}

/// Config struct
/// For convenience
#[derive(Deserialize)]
struct Config {
    apikey: String,
    locations: Vec<String>,
    dbconnectionstring: String,
}

impl Config {
    ///Creates a config instance.
    pub async fn new(config_file: PathBuf) -> Result<Config, io::Error> {
        Ok(serde_json::from_str(&fs::read_to_string(config_file).await.unwrap()).unwrap())
    }
}

async fn get_weather_openweathermap(api: &str, loc: &str) -> weather::Weather {
    //Gets the weather from openweathermap.com
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        loc, api
    );
    let response = get(&url).await.unwrap().text().await.unwrap();

    //converting to json so it can be printed
    let v: Value = serde_json::from_str(&response).unwrap();

    //returning the weather struct
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

/// Gets the city based on ip
async fn get_city(url: &str) -> String {
    get(url).await.unwrap().text().await.unwrap()
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //reading in loc and apikey
    let opt = Opt::from_args();

    //Gets the current configuration
    let config = Arc::new(Config::new(opt.config_file).await.unwrap());

    let dbconnectionstring = config.dbconnectionstring.clone();
    let client_options = mongodb::options::ClientOptions::parse(&dbconnectionstring)
        .await
        .unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("weatherlogger");

    //Getting the location
    if opt.isp_loc {
        let loc = get_city("http://ip-api.com/line/?fields=city").await;
        println!("{}", get_weather_openweathermap(&config.apikey, &loc).await);
    } else {
        //vector for containing the join_handles spawned from the tokio threads
        let mut futures = Vec::new();
        for loc in config.locations.clone() {
            //needed for move
            let apikey = config.apikey.clone();
            futures.push(tokio::spawn(async move {
                get_weather_openweathermap(&apikey, &loc).await
            }));
        }

        //Joining the futures
        let reses: Vec<weather::Weather> = try_join_all(futures)
            .await?
            .into_iter()
            .map(|res| res)
            .collect();

        //printing to stdout

        //inserting into the database
        let collection = db.collection::<weather::Weather>("weatherlog");
        collection.insert_many(&reses, None).await.unwrap();

        for res in &reses {
            println!("{}", res);
        }
    }
    Ok(())
}
