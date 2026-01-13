// wxr --configure --> like aws cli we will take ask user 1 time queions 1. his location 2.
// openwatehr api key....
//
// wxr --> will print! the whole weather info (temp, aql etc etc)
//
// wxr -temp,aqi --> will print only the flags.

use configstore::{AppUI, Configstore};
use dotenvy::dotenv; // for now im using .env for storing the api key later it will be done by .wxr config file
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
struct Coordinates {
    lat: f64,
    lon: f64,
}
#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    api_key: String,
    city: String,
    lat: Option<f64>,
    lon: Option<f64>,
}

// fetch user's lat, lon using their city name
fn fetch_coordinates(city: &str, api_key: &str) -> Result<Coordinates, Box<dyn std::error::Error>> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}",
        city, api_key
    );
    let res: Vec<Coordinates> = reqwest::blocking::get(url)?.json()?;
    Ok(Coordinates { ..res[0] }) // API returns a array of results from which we are taking only the 1st one
}

// fetch user's current lat, lon dynamically
fn fetch_user_coordinates() -> Result<Coordinates, Box<dyn std::error::Error>> {
    const URL: &str = "http://ip-api.com/json?fields=lat,lon";
    let res: Coordinates = reqwest::blocking::get(URL)?.json()?;
    Ok(res)
}

// Make a fn that  will store these lat, lon in users temp/.wxr folder
fn store_user_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_store = Configstore::new("wxr", AppUI::CommandLine).unwrap();
    let value = Config {
        api_key: "zyss".to_string(),
        city: "kolkata".to_string(),
        lat: None,
        lon: None,
    };
    config_store.set("config", value)?;
    
    Ok(()) // FIXME:  Path is bad : /home/agasta/.config/configstore-rs/wxr i dont wnat this configstore-rs....
}


// fetch user's weather info using their lat, lon
fn fetch_info(api_key: &str, lat: f64, lon: f64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    let res = reqwest::blocking::get(url)?.text()?;

    Ok(res) // TODO: use structs to deserilize this res.
}

// fn to get users info for making the config file in his dir.
// Res how aws-cli stores this info in user's system.

// main fun that will take flags
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Agasta!");

    // Temp code
    dotenv().ok();
    let api_key = env::var("API_KEY")?;
    let city = "Kolkata";

    let location = fetch_coordinates(city, &api_key)?;

    println!("{:#?}", location);

    println!("\n");

    println!("{:#?}", fetch_user_coordinates()?);

    let result = fetch_info(&api_key, location.lat, location.lon)?;

    println!("{}", result);
    
    store_user_config()?;

    Ok(())
}

// TODO: i will first fetch user location in main fun and store it in temp/ then req to api using those lat, long.... so we wont fetch teh location again & again... when laptop restart again temp lat, long is not avalible... fetch again.... If user explicitly set his cord uisng wxr --configure we will respect those cords (.wxr)
