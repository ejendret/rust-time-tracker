// src/project_utils.rs
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn proj_create(title: &str) -> Result<(), String> {
    let proj_path = format!(".{}", title);
    let proj_path = Path::new(&proj_path);

    let mut proj_file = File::create_new(&proj_path).map_err(|why| {
        format!(
            "[project_utils:proj_create] couldn't create {}: {}",
            proj_path.display(),
            why
        )
    })?;

    proj_file.write_all(title.as_bytes()).map_err(|why| {
        format!(
            "[project_utils:proj_create] couldn't write to {}: {}",
            proj_path.display(),
            why
        )
    })?;

    println!("Successfuly wrote {} to {}", title, proj_path.display());
    Ok(())
}
