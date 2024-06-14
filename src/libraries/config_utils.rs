// use std::path::Path;
// use std::fs;
// use std::io;

pub fn get_confirmation(query: &str) -> Result<(), String> {
    // Prompt the user with query
    println!("{}", query);
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).map_err(|why| {
        format!(
            "[config_utils:get_confirmation] couldn't read user input: {}",
            why
        )
    })?;

    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => Ok(()),
        _ => Err("Confirmation denied.".to_string()),
    }
}

// fn create_directory() -> Result<(), String> {
//     let config_dir = Path::new("~/.tracker");

//     std::fs::create_dir(config_dir).map_err(|why| {
//         format!(
//             "[config_utils:create_directory] couldn't create {}: {}",
//             config_dir.display(),
//             why
//         )
//     })
// }

// fn check_config() -> Result<(), String> {
//     let config_dir = Path::new("~/.tracker");
//     let config_path = Path::new("~/.tracker/config");

//     // Guard against no directory
//     if !Path::exists(config_dir) {
//         // Prompt user to create directory

//     }

// }

// fn retrieve_config() -> Result<(), String> {
//     let config_path = Path::new("~/.tracker/config");

//     // Try to read config file
// }
