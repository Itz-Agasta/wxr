// wxr --configure --> like aws cli we will take ask user 1 time queions 1. his location 2.
// openwatehr api key....
//
// wxr --> will print! the whole weather info (temp, aql etc etc)
//
// wxr -temp,aqi --> will print only the flags.
mod config;

use crate::config::{Config, set_user_config, };

use serde::{Deserialize};



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
   
   let cfg: Config = confy::load("wxr", "wxr_config")?; 
   
   let res = fetch_info(cfg.api_key.trim(), cfg.lat, cfg.lon)?;
   dbg!(res);

    // set_user_config()?;

    Ok(())
}

// TODO: i will first fetch user location in main fun and store it in temp/ then req to api using those lat, long.... so we wont fetch teh location again & again... when laptop restart again temp lat, long is not avalible... fetch again.... If user explicitly set his cord uisng wxr --configure we will respect those cords (.wxr)
