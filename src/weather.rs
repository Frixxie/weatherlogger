#![allow(dead_code)]
use csv::Reader;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::fmt;
use std::path::{Path, PathBuf};

//TODO: Make use of rust error system
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

    pub fn create_db_table(db: &Path) {
        let connection = sqlite::open(db).unwrap();
        connection
            .execute(
                "DROP TABLE IF EXISTS weather;
                CREATE TABLE IF NOT EXISTS weather (
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

    pub fn csv_to_db(csvfile: &Path, db: &Path) {
        let weather = Weather::read_from_csv(csvfile);
        let connection = sqlite::open(db).unwrap();
        for res in weather {
            connection.execute(format!("INSERT INTO weather (dt, name, country, lon, lat, main, desc, icon, sunrise, sunset, clouds, wind_speed, wind_deg, visibility, rain_1h, rain_3h, snow_1h, snow_3h, temp_min, temp_max, temp, feels_like, humidity, pressure) VALUES ({}, \"{}\", \"{}\", {}, {}, \"{}\", \"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});", res.dt, res.name, res.country, res.lon, res.lat, res.main, res.desc, res.icon, res.sunrise, res.sunset, res.clouds, res.wind_speed, res.wind_deg, res.visibility, res.rain_1h, res.rain_3h, res.snow_1h, res.snow_3h, res.temp_min, res.temp_max, res.temp, res.feels_like, res.humidity, res.pressure)).unwrap();
        }
    }

    pub fn read_from_csv(file: &Path) -> Vec<Weather> {
        let mut rdr = Reader::from_path(file).unwrap();
        let mut res = Vec::<Weather>::new();
        for result in rdr.deserialize() {
            let weather: Weather = result.unwrap();
            println!("{:?}", weather);
            res.push(weather);
        }
        res
    }

    pub fn write_to_db(&self, db: &Path) {
        let connection = sqlite::open(db).unwrap();
        connection.execute(format!("INSERT INTO weather (dt, name, country, lon, lat, main, desc, icon, sunrise, sunset, clouds, wind_speed, wind_deg, visibility, rain_1h, rain_3h, snow_1h, snow_3h, temp_min, temp_max, temp, feels_like, humidity, pressure) VALUES ({}, \"{}\", \"{}\", {}, {}, \"{}\", \"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});", self.dt, self.name, self.country, self.lon, self.lat, self.main, self.desc, self.icon, self.sunrise, self.sunset, self.clouds, self.wind_speed, self.wind_deg, self.visibility, self.rain_1h, self.rain_3h, self.snow_1h, self.snow_3h, self.temp_min, self.temp_max, self.temp, self.feels_like, self.humidity, self.pressure)).unwrap();
    }

    pub fn read_from_db(db: &Path) -> Vec<Weather> {
        let connection = sqlite::open(db).unwrap();
        let mut reses: Vec<Weather> = Vec::<Weather>::new();
        let mut cursor = connection
            .prepare("SELECT * FROM weather;")
            .unwrap()
            .cursor();
        while let Some(row) = cursor.next().unwrap() {
            let res: Weather = Weather::new(
                row[0].as_integer().unwrap() as u32,
                row[1].as_string().unwrap().to_string(),
                row[2].as_string().unwrap().to_string(),
                row[3].as_float().unwrap() as f32,
                row[4].as_float().unwrap() as f32,
                row[5].as_string().unwrap().to_string(),
                row[6].as_string().unwrap().to_string(),
                row[7].as_string().unwrap().to_string(),
                row[8].as_integer().unwrap() as u32,
                row[9].as_integer().unwrap() as u32,
                row[10].as_integer().unwrap() as u32,
                row[11].as_float().unwrap() as f32,
                row[12].as_integer().unwrap() as i32,
                row[13].as_integer().unwrap() as i32,
                row[14].as_float().unwrap() as f32,
                row[15].as_float().unwrap() as f32,
                row[16].as_float().unwrap() as f32,
                row[17].as_float().unwrap() as f32,
                row[18].as_float().unwrap() as f32,
                row[19].as_float().unwrap() as f32,
                row[20].as_float().unwrap() as f32,
                row[21].as_float().unwrap() as f32,
                row[22].as_integer().unwrap() as u32,
                row[23].as_integer().unwrap() as u32,
            );
            reses.push(res)
        }
        reses
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
        let file = PathBuf::from("weather_log.csv");
        let reses: Vec<Weather> = Weather::read_from_csv(&file);
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
        let db = PathBuf::from("db.sqlite");
        Weather::create_db_table(&db);
        Weather::csv_to_db(&PathBuf::from("weather_log.csv"), &db);
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
        let mut cursor = connection
            .prepare("SELECT * FROM weather;")
            .unwrap()
            .cursor();
        let row = cursor.next().unwrap().unwrap();
        let new_weather: Weather = Weather::new(
            row[0].as_integer().unwrap() as u32,
            row[1].as_string().unwrap().to_string(),
            row[2].as_string().unwrap().to_string(),
            row[3].as_float().unwrap() as f32,
            row[4].as_float().unwrap() as f32,
            row[5].as_string().unwrap().to_string(),
            row[6].as_string().unwrap().to_string(),
            row[7].as_string().unwrap().to_string(),
            row[8].as_integer().unwrap() as u32,
            row[9].as_integer().unwrap() as u32,
            row[10].as_integer().unwrap() as u32,
            row[11].as_float().unwrap() as f32,
            row[12].as_integer().unwrap() as i32,
            row[13].as_integer().unwrap() as i32,
            row[14].as_float().unwrap() as f32,
            row[15].as_float().unwrap() as f32,
            row[16].as_float().unwrap() as f32,
            row[17].as_float().unwrap() as f32,
            row[18].as_float().unwrap() as f32,
            row[19].as_float().unwrap() as f32,
            row[20].as_float().unwrap() as f32,
            row[21].as_float().unwrap() as f32,
            row[22].as_integer().unwrap() as u32,
            row[23].as_integer().unwrap() as u32,
        );
        assert_eq!(new_weather, weather);
    }

    #[test]
    fn test_sqlite_write() {
        let db = PathBuf::from("db.sqlite");
        Weather::create_db_table(&db);
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
        Weather::write_to_db(&weather, &db);
        let connection = sqlite::open(&db).unwrap();
        let mut cursor = connection
            .prepare("SELECT * FROM weather;")
            .unwrap()
            .cursor();
        let row = cursor.next().unwrap().unwrap();
        let new_weather: Weather = Weather::new(
            row[0].as_integer().unwrap() as u32,
            row[1].as_string().unwrap().to_string(),
            row[2].as_string().unwrap().to_string(),
            row[3].as_float().unwrap() as f32,
            row[4].as_float().unwrap() as f32,
            row[5].as_string().unwrap().to_string(),
            row[6].as_string().unwrap().to_string(),
            row[7].as_string().unwrap().to_string(),
            row[8].as_integer().unwrap() as u32,
            row[9].as_integer().unwrap() as u32,
            row[10].as_integer().unwrap() as u32,
            row[11].as_float().unwrap() as f32,
            row[12].as_integer().unwrap() as i32,
            row[13].as_integer().unwrap() as i32,
            row[14].as_float().unwrap() as f32,
            row[15].as_float().unwrap() as f32,
            row[16].as_float().unwrap() as f32,
            row[17].as_float().unwrap() as f32,
            row[18].as_float().unwrap() as f32,
            row[19].as_float().unwrap() as f32,
            row[20].as_float().unwrap() as f32,
            row[21].as_float().unwrap() as f32,
            row[22].as_integer().unwrap() as u32,
            row[23].as_integer().unwrap() as u32,
        );
        assert_eq!(new_weather, weather);
    }

    #[test]
    fn test_sqlite_read() {
        let db = PathBuf::from("db.sqlite");
        Weather::create_db_table(&db);
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
        Weather::write_to_db(&weather, &db);
        let reses = Weather::read_from_db(&db);
        assert_eq!(reses[0], weather)
    }
}