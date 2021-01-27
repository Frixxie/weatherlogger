use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;

fn get_weather(api: &str, loc: &str, client: &Client) -> String {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        loc, api
    );
    let response = client.get(&url).send().unwrap().text().unwrap();

    //converting to json so it can be printed
    let v: Value = serde_json::from_str(&response).unwrap();

    let weather = format!(
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
    );
    weather
}

fn main() {
    //reading in loc and apikey
    let locs = fs::read_to_string("/home/fredrik/projects/weather/location").unwrap();
    let api = fs::read_to_string("/home/fredrik/projects/weather/apikey").unwrap();

    //trimming
    locs.trim_matches(char::is_control).to_string();
    api.trim_matches(char::is_control).to_string();

    let client = Client::new();

    //looping through locs
    for loc in locs.split_whitespace() {
        println!("{}", get_weather(&api, &loc, &client))
    }
}
