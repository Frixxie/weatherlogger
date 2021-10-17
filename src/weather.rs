#![allow(dead_code)]
use csv::Reader;
use plotters::prelude::*;
use rayon::prelude::*;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;
use std::path::Path;

//TODO: Make use of rust error system
#[derive(Debug, Deserialize, PartialEq, Clone)]
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

    /// TODO: implememnt from str trait i guess
    pub fn new_from_json(weather_json: String) -> Weather {
        serde_json::from_str(&weather_json).unwrap()
    }

    /// calculates the mean temp
    /// Callers resposability to have filtered the location of data!
    pub fn mean_temp(weather: &[Weather]) -> Option<f32> {
        if !weather.is_empty() {
            return Some(
                weather
                    .into_par_iter()
                    .map(|weather| weather.temp)
                    .sum::<f32>()
                    / weather.len() as f32,
            );
        }
        None
    }

    /// Finds the smallest value in the vec of f32s
    fn min_f32(vec: &[f32]) -> f32 {
        let mut min = std::f32::MAX;
        for val in vec.iter() {
            if val < &min {
                min = *val;
            }
        }
        min
    }

    /// Finds the largest value in the vec of f32s
    fn max_f32(vec: &[f32]) -> f32 {
        let mut max = std::f32::MIN;
        for val in vec.iter() {
            if val > &max {
                max = *val;
            }
        }
        max
    }

    /// Filters out weather matching name
    pub fn filter(weathers: &[Weather], name: &str) -> Vec<Weather> {
        weathers
            .to_owned()
            .into_par_iter()
            .filter(|weather| weather.name == name)
            .map(|weather| weather)
            .collect()
    }

    /// Get the locations by getting the list of locations sorting it and then
    /// running dedup on the list to remove the duplicates
    pub fn get_locations(weathers: &[Weather]) -> Vec<String> {
        let mut res: Vec<String> = weathers
            .par_iter()
            .map(|weather| weather.name.clone())
            .collect();
        res.par_sort_unstable();
        res.dedup();
        res
    }

    /// PLOTS the temperature from weather, which needs to be filterd
    /// The code is based on the examples provided by the plotters crate
    pub fn create_temp_plot(weathers: &[Weather], filename: &Path) {
        let root = BitMapBackend::new(filename, (1280, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        // gets the time and temperature from weathers
        let dts: Vec<f32> = weathers
            .into_par_iter()
            .map(|weather| weather.dt as f32)
            .collect();
        let temps: Vec<f32> = weathers
            .into_par_iter()
            .map(|weather| weather.temp)
            .collect();
        let feels_likes: Vec<f32> = weathers
            .into_par_iter()
            .map(|weather| weather.feels_like)
            .collect();

        // Finds min and max values needed for plot
        let min_dt = Weather::min_f32(&dts);
        let max_dt = Weather::max_f32(&dts);
        let min_temps = Weather::min_f32(&temps);
        let max_temps = Weather::max_f32(&temps);

        // creates chart instance
        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("Temperature in {}", weathers[0].name.to_owned()),
                ("sans-serif", 30).into_font(),
            )
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(min_dt..max_dt, min_temps..max_temps)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        // creates the points to be plotted
        let mut points: Vec<(f32, f32)> = dts
            .to_vec()
            .into_par_iter()
            .zip(temps.into_par_iter())
            .map(|val| (val.0.to_owned(), val.1.to_owned()))
            .collect();

        // Draws points
        chart
            .draw_series(LineSeries::new(points, &RED))
            .unwrap()
            .label("Temperature")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        points = dts
            .to_vec()
            .into_par_iter()
            .zip(feels_likes.into_par_iter())
            .map(|val| (val.0.to_owned(), val.1.to_owned()))
            .collect();

        chart
            .draw_series(LineSeries::new(points, &GREEN))
            .unwrap()
            .label("Feels like")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        // updates and fixes(?)
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }


    pub fn read_from_csv(file: &Path) -> Result<Vec<Weather>, csv::Error> {
        let mut rdr = Reader::from_path(file).unwrap();
        let mut res = Vec::<Weather>::new();
        for result in rdr.deserialize() {
            match result {
                Ok(weather) => res.push(weather),
                Err(error) => return Err(error),
            }
        }
        Ok(res)
    }


    pub fn write_to_csv(&self, csvfile: &Path) -> Result<(), String> {
        todo!()
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

impl Error for Weather {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_csvfile() {
        let file = PathBuf::from("weather_log.csv");
        let reses: Vec<Weather> = Weather::read_from_csv(&file).unwrap();
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
    fn test_mean_test() {
        let file = PathBuf::from("weather_log.csv");
        let reses: Vec<Weather> = Weather::read_from_csv(&file).unwrap();
        println!("{:?}", reses);
        let res = Weather::mean_temp(&reses).unwrap();
        assert_eq!(res, -5.77)
    }
}
