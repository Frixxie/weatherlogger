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
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            v["dt"],
            v["name"],
            v["sys"]["country"],
            v["weather"][0]["description"],
            v["sys"]["sunrise"],
            v["sys"]["sunset"],
            v["clouds"]["all"],
            v["wind"]["speed"],
            v["wind"]["deg"],
            v["main"]["temp_min"],
            v["main"]["temp_max"],
            v["main"]["temp"],
            v["main"]["humidity"],
            v["main"]["pressure"],
        );

    }

    //println!("{:#?}", res);
}
