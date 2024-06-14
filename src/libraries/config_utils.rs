use std::fs;
use std::path::Path;
use users::get_current_username;
// use std::io;

pub fn get_confirmation(query: &str) -> Result<(), String> {
    // Prompt the user with query
    println!("{}", query);
    let mut input = String::new();

    // Get user input
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|why| format!("[config_utils:get_confirmation] {}", why))?;

    // If yes or y, return success, else error
    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => Ok(()),
        _ => Err("confirmation denied.".to_string()),
    }
}

fn create_directory(path: &Path) -> Result<(), String> {
    std::fs::create_dir(path).map_err(|why| format!("[config_utils:create_directory] {}", why))
}

fn create_file(path: &Path) -> Result<(), String> {
    // Create file
    fs::File::create(path).map_err(|why| format!("[config_utils:check_config] {}", why))?;

    // TODO: write something into file

    Ok(())
}

pub fn check_config() -> Result<(), String> {
    // Retrieve username
    let user = match get_current_username() {
        Some(username) => Ok(username),
        None => Err("No user found"),
    }?;

    let user_str = user
        .into_string()
        .unwrap_or_else(|os_string| os_string.to_string_lossy().into_owned());
    let user = format!("{}", user_str);

    println!("Username is: {}", user);

    let config_dir = format!("/home/{}/.tracker", user);
    let config_dir_path = Path::new(&config_dir);
    let config_file = format!("/home/{}/.tracker/config", user);
    let config_file_path = Path::new(&config_file);

    // If no config dir
    if !Path::exists(config_dir_path) {
        // Prompt user to create directory, error if issue or negative
        let query = format!(
            "Configuration directory needed. Create at {} ? Enter yes or y to confirm.",
            config_dir
        );
        get_confirmation(&query).map_err(|why| format!("[config_utils:check_config]: {}", why))?;

        // Create dir
        create_directory(config_dir_path)
            .map_err(|why| format!("[config_utils:check_config] {}", why))?;

        println!("Configuration directory created successfully.")
    }

    // If no config file
    if !Path::exists(config_file_path) {
        // Prompt user to create directory, error if issue or negative
        let query = format!(
            "Configuration file needed. Create at {} ? Enter yes or y to confirm.",
            config_file
        );
        get_confirmation(&query).map_err(|why| format!("[config_utils:check_config]: {}", why))?;

        create_file(config_file_path)
            .map_err(|why| format!("[config_utils:check_config: {}", why))?;

        println!("Configuration file created successfully.")
    }

    Ok(())
}

// fn retrieve_config() -> Result<(), String> {
//     let config_path = Path::new("/home/.tracker/config");

//     // Try to read config file
// }
