use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use users::get_current_username;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Config {
    current_proj: String,
    projects: Vec<String>,
}

impl Config {
    pub fn new(current_proj: Option<String>, projects: Option<Vec<String>>) -> Config {
        Config {
            current_proj: current_proj.unwrap_or_else(|| "none".to_string()),
            projects: projects.unwrap_or_else(|| vec![]),
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let config_file = File::open(path)?;
        let config: Config = from_reader(config_file)?;
        Ok(config)
    }

    pub fn to_file(&self, path: &str) -> io::Result<()> {
        let config_file = File::create(path)?;
        to_writer_pretty(config_file, &self)?;
        Ok(())
    }

    pub fn set_current_proj(&mut self, new_proj: String) -> Result<(), String> {
        if self.projects.contains(&new_proj) {
            self.current_proj = new_proj;
            Ok(())
        } else {
            Err(format!("No project by name {}", new_proj))
        }
    }

    pub fn get_current_proj(&self) -> &str {
        &self.current_proj
    }

    pub fn add_project(&mut self, new_proj: String) {
        if !self.projects.contains(&new_proj) {
            self.projects.push(new_proj);
        }
    }

    pub fn remove_project(&mut self, proj: &str) {
        self.projects.retain(|x| x != proj);
        if self.current_proj == proj {
            self.current_proj = "none".to_string();
        }
    }

    pub fn get_projects(&self) -> &Vec<String> {
        &self.projects
    }
}

pub fn get_config_location() -> Result<String, String> {
    // Retrieve username
    let user = match get_current_username() {
        Some(username) => Ok(username),
        None => Err("Unable to retrieve username."),
    }?;

    let user_str = user
        .into_string()
        .unwrap_or_else(|os_string| os_string.to_string_lossy().into_owned());
    Ok(format!("/home/{}/.tracker", user_str))
}

pub fn get_confirmation<R: Read>(query: &str, reader: R) -> Result<(), String> {
    // Prompt the user with query
    println!("{}", query);
    let mut input = String::new();

    // Get user input
    let mut buffered = io::BufReader::new(reader);
    buffered
        .read_line(&mut input)
        .map_err(|why| format!("[config_utils:get_confirmation]: {}", why))?;

    // If yes or y, return success, else error
    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => Ok(()),
        _ => Err("confirmation denied.".to_string()),
    }
}

fn create_directory(path: &Path) -> Result<(), String> {
    std::fs::create_dir(path).map_err(|why| format!("[config_utils:create_directory]: {}", why))
}

pub fn build_config(config_dir: &str) -> Result<(), String> {
    let config_dir_path = Path::new(&config_dir);
    let config_file = format!("{}/config.JSON", config_dir);
    let config_file_path = Path::new(&config_file);

    // If no config dir
    if !Path::exists(config_dir_path) {
        // Create dir
        create_directory(config_dir_path)
            .map_err(|why| format!("[config_utils:check_config]: {}", why))?;

        println!("Configuration directory created successfully.")
    }

    // If no config file
    if !Path::exists(config_file_path) {
        // Prompt user to create directory, error if issue or negative

        let config = Config::new(None, None);
        config
            .to_file(&config_file)
            .map_err(|why| format!("Failed to create config file {}", why))?;

        println!("Configuration file created successfully.");
    }
    Ok(())
}

pub fn config_present(config_dir: &str) -> bool {
    // Construct paths
    let config_dir_path = Path::new(&config_dir);
    let config_file = format!("{}/config.JSON", config_dir);
    let config_file_path = Path::new(&config_file);

    config_dir_path.exists() && config_file_path.exists()
}
