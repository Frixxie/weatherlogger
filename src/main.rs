use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use structopt::StructOpt;

use std::path::PathBuf;

use std::io;

#[derive(Debug, StructOpt)]
#[structopt(name = "weatherlogger", about = "Logs the weather from openweathermap")]
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

fn get_weather(api: &str, loc: &str, client: &Client) -> String {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        loc, api
    );
    let response = client.get(&url).send().unwrap().text().unwrap();

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

fn get_city(url: &str, client: &Client) -> String {
    client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap()
        .trim_matches(char::is_control)
        .to_string()
}

fn main() -> Result<(), io::Error> {
    //reading in loc and apikey
    let opt = Opt::from_args();
    let api = fs::read_to_string(opt.apikey_file)?;

    //trimming
    api.trim_matches(char::is_control).to_string();

    let client = Client::new();

    let locs = if opt.isp_loc {
        vec![get_city("http://ip-api.com/line/?fields=city", &client)]
    } else {
        fs::read_to_string(opt.locations_file)?
            .trim_matches(char::is_control)
            .to_string()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    };

    for loc in locs {
        println!("{}", get_weather(&api, &loc, &client))
    }

    Ok(())
}
