use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    pub fn new(name: &str) -> Vec<Weather> {
        let mut rdr = Reader::from_path(name).unwrap();
        let mut res = Vec::<Weather>::new();
        for result in rdr.deserialize() {
            let weather: Weather = result.unwrap();
            println!("{:?}", weather);
            res.push(weather);
        }
        res
    }
}
