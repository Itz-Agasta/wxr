// wxr --configure --> like aws cli we will take ask user 1 time queions 1. his location 2.
// openwatehr api key....
//
// wxr --> will print! the whole weather info (temp, aql etc etc)
//
// wxr -temp,aqi --> will print only the flags.
mod config;
use crate::config::{Config, set_user_config};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub main: String,
    pub description: String,
}
#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub sunrise: i32,
    pub sunset: i32,
}

#[derive(Deserialize, Debug)]
struct Info {
    weather: Vec<Weather>,
    main: Main,
    visibility: i32,
    sys: Sys,
}

// TODO: Make this async & fetch AQI too
// fetch user's weather info using their lat, lon
fn fetch_info(api_key: &str, lat: f64, lon: f64) -> Result<Info, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );
    let res = reqwest::blocking::get(url)?.json()?;
    Ok(res)
}

// main fun that will take flags
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Agasta!");

    let args: Vec<String> = env::args().collect();

    // Check if user wants to run config setup
    if args.len() > 1 && args[1] == "config" {
        set_user_config()?;
        return Ok(());
    }

    let cfg: Config = confy::load("wxr", "wxr_config")?;

    // If config is not set up, run setup
    if cfg.api_key.is_empty() || (cfg.lat == 0.0 && cfg.lon == 0.0) {
        set_user_config()?;
    } else {
        match fetch_info(cfg.api_key.trim(), cfg.lat, cfg.lon) {
            Ok(res) => {
                println!("\n🌤️  Weather Information");
                println!("━━━━━━━━━━━━━━━━━━━━━━");
                println!("Condition: {}", res.weather[0].main);
                println!("Temperature: {:.1}°C", res.main.temp - 273.15); // Convert Kelvin to Celsius
                println!("Humidity: {}%", res.main.humidity);
                println!("Pressure: {} hPa", res.main.pressure);
                println!("Visibility: {} m", res.visibility);
                println!("Sun-Rise: {} AM", res.sys.sunrise);
            }
            Err(e) => {
                eprintln!("Failed to fetch weather info. Please verify your API key.");
                eprintln!("You can reconfigure by running: wxr config");
                return Err(e);
            }
        }
    }

    Ok(())
}
