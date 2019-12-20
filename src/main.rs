extern crate reqwest;
use serde_json::Value;
use std::fs;

fn main() {
    let locs = fs::read_to_string("/home/fredrik/projects/weather/locations").unwrap();
    let api = fs::read_to_string("/home/fredrik/projects/weather/openweather-api").unwrap();

    locs.trim_matches(char::is_control).to_string();
    api.trim_matches(char::is_control).to_string();

    for loc in locs.split_whitespace() {
        //println!("{}", loc);

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
            loc, api
        );

        let res = reqwest::get(url.as_str()).unwrap().text().unwrap();

        //println!("{}", res);

        let v: Value = serde_json::from_str(res.as_str()).unwrap();

        println!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
            v["clouds"]["all"],
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

    }

    //println!("{:#?}", res);
}
