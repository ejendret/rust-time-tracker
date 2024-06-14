use std::path::Path;
use std::fs;
// use std::io;

pub fn get_confirmation(query: &str) -> Result<(), String> {
    // Prompt the user with query
    println!("{}", query);
    let mut input = String::new();

    // Get user input
    std::io::stdin().read_line(&mut input).map_err(|why| {
        format!(
            "[config_utils:get_confirmation] {}",
            why
        )
    })?;

    // If yes or y, return success, else error
    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => Ok(()),
        _ => Err("confirmation denied.".to_string()),
    }
}

fn create_directory() -> Result<(), String> {
    let config_dir = Path::new("/home/ejendret/.tracker");

    std::fs::create_dir(config_dir).map_err(|why| {
        format!(
            "[config_utils:create_directory] {}",
            why
        )
    })
}

pub fn check_config() -> Result<(), String> {
    let config_dir = Path::new("/home/ejendret/.tracker");
    let config_path = Path::new("/home/ejendret/.tracker/config");

    // If no config dir
    if !Path::exists(config_dir) {
        // Prompt user to create directory, error if issue or negative
        get_confirmation("Configuration directory needed. Create in /home? Enter yes or y to confirm.").map_err(|why| {
            format!(
                "[config_utils:check_config]: {}",
                why
            )
        })?;

        // Create dir
        create_directory().map_err(|why| {
            format!(
                "[config_utils:check_config] {}",
                why
            )
        })?;

        println!("Configuration directory created successfully.")
    }

    // If no config file
    if !Path::exists(config_path) {
        // Prompt user to create directory, error if issue or negative
        get_confirmation("Configuration file needed. Create in /home? Enter yes or y to confirm.").map_err(|why| {
            format!(
                "[config_utils:check_config]: {}",
                why
            )
        })?;

        // Create file
        fs::File::create(config_path).map_err(|why| {
            format!(
                "[config_utils:check_config] {}",
                why
            )
        })?;

        println!("Configuration file created successfully.")
    }

    Ok(())
}

// fn retrieve_config() -> Result<(), String> {
//     let config_path = Path::new("/home/.tracker/config");

//     // Try to read config file
// }
