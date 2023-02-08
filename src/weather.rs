use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
    dt: u32,
    name: String,
    country: String,
    lon: f32,
    lat: f32,
    main: String,
    desc: String,
    icon: String,
    sunrise: u32,
    sunset: u32,
    clouds: u32,
    wind_speed: f32,
    wind_deg: i32,
    visibility: i32,
    rain_1h: f32,
    rain_3h: f32,
    snow_1h: f32,
    snow_3h: f32,
    temp_min: f32,
    temp_max: f32,
    temp: f32,
    feels_like: f32,
    humidity: u32,
    pressure: u32,
}

impl Weather {
    pub fn new(
        dt: u32,
        name: String,
        country: String,
        lon: f32,
        lat: f32,
        main: String,
        desc: String,
        icon: String,
        sunrise: u32,
        sunset: u32,
        clouds: u32,
        wind_speed: f32,
        wind_deg: i32,
        visibility: i32,
        rain_1h: f32,
        rain_3h: f32,
        snow_1h: f32,
        snow_3h: f32,
        temp_min: f32,
        temp_max: f32,
        temp: f32,
        feels_like: f32,
        humidity: u32,
        pressure: u32,
    ) -> Weather {
        Weather {
            dt,
            name,
            country,
            lon,
            lat,
            main,
            desc,
            icon,
            sunrise,
            sunset,
            clouds,
            wind_speed,
            wind_deg,
            visibility,
            rain_1h,
            rain_3h,
            snow_1h,
            snow_3h,
            temp_min,
            temp_max,
            temp,
            feels_like,
            humidity,
            pressure,
        }
    }
}

impl TryFrom<serde_json::Value> for Weather {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let dt = value["dt"].as_u64().unwrap() as u32;
        let name = value["name"].as_str().unwrap().to_string();
        let country = value["sys"]["country"].as_str().unwrap().to_string();
        let lon = value["coord"]["lon"].as_f64().unwrap() as f32;
        let lat = value["coord"]["lat"].as_f64().unwrap() as f32;
        let main = value["weather"][0]["main"].as_str().unwrap().to_string();
        let desc = value["weather"][0]["description"]
            .as_str()
            .unwrap()
            .to_string();
        let icon = value["weather"][0]["icon"].as_str().unwrap().to_string();
        let sunrise = value["sys"]["sunrise"].as_u64().unwrap() as u32;
        let sunset = value["sys"]["sunset"].as_u64().unwrap() as u32;
        let clouds = value["clouds"]["all"].as_u64().unwrap() as u32;
        let wind_speed = value["wind"]["speed"].as_f64().unwrap() as f32;
        let wind_deg = value["wind"]["deg"].as_i64().unwrap() as i32;
        let visibility = value["visibility"].as_i64().unwrap() as i32;
        let rain_1h = value["rain"]["1h"].as_f64().unwrap_or(0.0) as f32;
        let rain_3h = value["rain"]["3h"].as_f64().unwrap_or(0.0) as f32;
        let snow_1h = value["snow"]["1h"].as_f64().unwrap_or(0.0) as f32;
        let snow_3h = value["snow"]["3h"].as_f64().unwrap_or(0.0) as f32;
        let temp_min = value["main"]["temp_min"].as_f64().unwrap() as f32;
        let temp_max = value["main"]["temp_max"].as_f64().unwrap() as f32;
        let temp = value["main"]["temp"].as_f64().unwrap() as f32;
        let feels_like = value["main"]["feels_like"].as_f64().unwrap() as f32;
        let humidity = value["main"]["humidity"].as_u64().unwrap() as u32;
        let pressure = value["main"]["pressure"].as_u64().unwrap() as u32;

        Ok(Weather::new(
            dt, name, country, lon, lat, main, desc, icon, sunrise, sunset, clouds, wind_speed,
            wind_deg, visibility, rain_1h, rain_3h, snow_1h, snow_3h, temp_min, temp_max, temp,
            feels_like, humidity, pressure,
        ))
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.dt,
            self.name,
            self.country,
            self.lon,
            self.lat,
            self.main,
            self.desc,
            self.icon,
            self.sunrise,
            self.sunset,
            self.clouds,
            self.wind_speed,
            self.wind_deg,
            self.visibility,
            self.rain_1h,
            self.rain_3h,
            self.snow_1h,
            self.snow_3h,
            self.temp_min,
            self.temp_max,
            self.temp,
            self.feels_like,
            self.humidity,
            self.pressure
        )
    }
}
