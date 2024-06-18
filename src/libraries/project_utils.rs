// src/project_utils.rs
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;
use crate::task_utils::Task;

#[derive(Debug, Deserialize, Serialize)]
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
            total_duration: 0
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
                Err(e) => Err(format!("Failed to write project data to file {} {}", path.display(), e)),
            }
        },
        Err(e) => {
            Err(format!("Failed to create project file {}", e))
        }
    }
}

pub fn retrieve_project(path: &Path) -> Project {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not read file");
    let project: Project = serde_json::from_reader(file).unwrap();
    project
}