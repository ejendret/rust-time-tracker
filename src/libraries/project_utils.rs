// src/project_utils.rs
use crate::task_utils::Task;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Project {
    pub title: String,
    pub tasks: Vec<Task>,
    pub total_duration: i64,
}

impl Project {
    pub fn new(title: String) -> Project {
        Project {
            title: title,
            tasks: Vec::new(),
            total_duration: 0,
        }
    }
}

pub fn create_project(title: &str, path: &Path) -> Result<String, String> {
    // Create and serialize project structure
    let proj = Project::new(title.to_string());

    // Try to create project file
    match File::create_new(path) {
        Ok(proj_file) => {
            // Try to write project to file
            let mut writer = BufWriter::new(proj_file);
            match serde_json::to_writer(&mut writer, &proj) {
                Ok(_) => Ok(format!("Created project {} at {}", title, path.display())),
                Err(e) => Err(format!(
                    "Failed to write project data to file {} {}",
                    path.display(),
                    e
                )),
            }
        }
        Err(e) => Err(format!("Failed to create project file {}", e)),
    }
}

pub fn write_project(path: &Path, proj: &Project) -> Result<String, String> {
    match OpenOptions::new()
        .write(true) // Enable writing
        .truncate(true) // Truncate the file upon opening
        .open(path)
    {
        // Do not create if it doesn't exist
        Ok(proj_file) => {
            let mut writer = BufWriter::new(proj_file);
            match serde_json::to_writer(&mut writer, &proj) {
                Ok(_) => Ok(format!("Updated project {}", proj.title)),
                Err(e) => Err(format!("Failed to update project {}: {}", proj.title, e)),
            }
        }
        Err(e) => Err(format!(
            "Failed to open project {}: {}. Ensure the file exists before updating.",
            proj.title, e
        )),
    }
}

pub fn read_project(path: &Path) -> Result<Project, String> {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(project) => Ok(project),
            Err(e) => Err(format!("Could not read file into struct: {}", e)),
        },
        Err(e) => Err(format!("Could not open file: {}", e)),
    }
}
