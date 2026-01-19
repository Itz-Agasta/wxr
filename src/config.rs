use prompted::input;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}
#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub api_key: String,
    pub city: Option<String>,
    pub lat: f64,
    pub lon: f64,
}
// Make a fn that  will store these lat, lon in users ~/.config/wxr folder
pub fn set_user_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg: Config = confy::load("wxr", "wxr_config")?;

    // API KEY
    loop {
        let api_key = input!("Enter your OpenWeather API key: ");
        if !api_key.trim().is_empty() {
            cfg.api_key = api_key;
            break;
        } else {
            println!("API key cannot be empty. Please try again.\n");
        }
    }

    // LOCATION METHOD
    println!("\nHow do you want to set your location?");
    println!("1) Auto-detect using IP");
    println!("2) Enter city manually");

    let choice: String = input!("Choose (1 or 2): ");

    match choice.trim() {
        "1" => match fetch_user_coordinates() {
            Ok(coord) => {
                println!("Location detected successfully!!");
                cfg.lat = coord.lat;
                cfg.lon = coord.lon;
            }
            Err(e) => {
                eprintln!("Failed to auto-detect location: {e}");
                println!("Falling back to manual city input...\n");

                let city = input!("Enter your city name: ");
                if city.trim().is_empty() {
                    return Err("City name cannot be empty".into());
                }

                println!("Looking up city coordinates...");

                match fetch_coordinates(city.trim(), &cfg.api_key) {
                    Ok(coord) => {
                        println!("City found!");
                        cfg.city = Some(city);
                        cfg.lat = coord.lat;
                        cfg.lon = coord.lon;
                    }
                    Err(e) => {
                        eprintln!("Failed to find city: {e}");
                        return Err("Setup failed. Please check your API key and try again.".into());
                    }
                }
            }
        },
        "2" => {
            let city = input!("\nEnter your city name: ");
            if city.trim().is_empty() {
                return Err("City name cannot be empty".into());
            }

            println!("Looking up city coordinates...");

            match fetch_coordinates(city.trim(), &cfg.api_key) {
                Ok(coord) => {
                    println!("City found!");
                    cfg.city = Some(city);
                    cfg.lat = coord.lat;
                    cfg.lon = coord.lon;
                }
                Err(e) => {
                    eprintln!("Failed to find city: {e}");
                    eprintln!("\nPossible reasons:");
                    eprintln!("  - Invalid API key");
                    eprintln!("  - City name not recognized");
                    eprintln!("  - Network issue");
                    return Err(
                        "Setup failed. Please verify your API key and city name, then try again."
                            .into(),
                    );
                }
            }
        }
        _ => {
            return Err("Invalid choice. Please run setup again.".into());
        }
    }

    // Validate coordinates were set
    if cfg.lat == 0.0 || cfg.lon == 0.0 {
        return Err("Failed to set location coordinates".into());
    }

    confy::store("wxr", "wxr_config", cfg)?;
    println!("\n✓ wxr setup completed successfully!");
    println!("You can now run: wxr\n");

    Ok(())
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
