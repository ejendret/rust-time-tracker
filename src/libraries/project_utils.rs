// src/project_utils.rs
use serde::{Deserialize, Serialize};
use std::fs::File;
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

pub fn create_project(title: &str, path: &Path) -> Result<(), String> {
    // Create and serialize project structure
    let proj = Project::new(title.to_string());
       
    // Try to create project file
    let proj_file = File::create_new(path).map_err(|why| {
        format!(
            "[project_utils:proj_create] couldn't create {}: {}",
            path.display(),
            why
        )
    })?;
    
    // Try to write project to file
    let mut writer = BufWriter::new(proj_file);
    serde_json::to_writer(&mut writer, &proj).map_err(|why| {
        format!(
            "[project_utils:proj_create] couldn't write to {}: {}",
            path.display(),
            why
        )
    })?;

    Ok(())
}
