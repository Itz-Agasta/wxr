use crate::{Config, fetch_coordinates, fetch_user_coordinates};
use prompted::input;

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
                cfg.lat = Some(coord.lat);
                cfg.lon = Some(coord.lon);
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
                        cfg.city = Some(city.trim().to_string());
                        cfg.lat = Some(coord.lat);
                        cfg.lon = Some(coord.lon);
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
                    cfg.city = Some(city.trim().to_string());
                    cfg.lat = Some(coord.lat);
                    cfg.lon = Some(coord.lon);
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
    if cfg.lat.is_none() || cfg.lon.is_none() {
        return Err("Failed to set location coordinates".into());
    }

    confy::store("wxr", "wxr_config", cfg)?;
    println!("\n✓ wxr setup completed successfully!");
    println!("You can now run: wxr\n");

    Ok(())
}
