use csv::Reader;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq)]
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

    pub fn create_db_table(db: PathBuf) {
        let connection = sqlite::open(db).unwrap();
        connection
            .execute(
                "CREATE TABLE weather (
                dt INTEGER,
                name TEXT,
                country TEXT,
                lon FLOAT,
                lat FLOAT,
                main TEXT,
                desc TEXT,
                icon TEXT,
                sunrise INTEGER,
                sunset INTEGER,
                clouds INTEGER,
                wind_speed FLOAT,
                wind_deg INTEGER,
                visibility INTEGER,
                rain_1h FLOAT,
                rain_3h FLOAT,
                snow_1h FLOAT,
                snow_3h FLOAT,
                temp_min FLOAT,
                temp_max FLOAT,
                temp FLOAT,
                feels_like FLOAT,
                humidity INTEGER,
                pressure INTEGER
                );",
            )
            .unwrap();
    }

    pub fn populate_db(csvfile: PathBuf, db: PathBuf) {
        let weather = Weather::read_from_csv(csvfile);
        let connection = sqlite::open(db).unwrap();
        for res in weather {
            connection
                .execute(format!(
                    "INSERT INTO weather (
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
                    ) VALUES {}",
                    res
                ))
                .unwrap();
        }
    }

    pub fn read_from_csv(file: PathBuf) -> Vec<Weather> {
        let mut rdr = Reader::from_path(file).unwrap();
        let mut res = Vec::<Weather>::new();
        for result in rdr.deserialize() {
            let weather: Weather = result.unwrap();
            println!("{:?}", weather);
            res.push(weather);
        }
        res
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csvfile() {
        let file = PathBuf::new().push("weather_log.csv");
        let reses: Vec<Weather> = Weather::read_from_csv(file);
        let weather: Weather = Weather::new(
            1615067637,
            "Tromsø".to_string(),
            "NO".to_string(),
            18.957,
            69.6496,
            "Snow".to_string(),
            "snow".to_string(),
            "13n".to_string(),
            1615009623,
            1615046646,
            90,
            2.57,
            170,
            1100,
            0.0,
            0.0,
            0.57,
            0.0,
            -6.0,
            -5.56,
            -5.77,
            -10.35,
            93,
            999,
        );
        assert_eq!(weather, reses[0])
    }

    #[test]
    fn test_sqlite() {
        let db = "db.sqlite";
        Weather::create_db_table(&db);
        Weather::populate_db("weather_log.csv", &db);
        let weather: Weather = Weather::new(
            1615067637,
            "Tromsø".to_string(),
            "NO".to_string(),
            18.957,
            69.6496,
            "Snow".to_string(),
            "snow".to_string(),
            "13n".to_string(),
            1615009623,
            1615046646,
            90,
            2.57,
            170,
            1100,
            0.0,
            0.0,
            0.57,
            0.0,
            -6.0,
            -5.56,
            -5.77,
            -10.35,
            93,
            999,
        );
        let connection = sqlite::open(&db).unwrap();
        //connection.iterate("SELECT * FROM weather", |pairs|);
    }
}
