use std::{
    fs::{self, remove_file, File},
    path::Path,
};

use chrono::Utc;
// tests/proj_tests.rs
use rust_time_tracker::{
    project_utils::{self, read_project, write_project, Project},
    task_utils::{Session, Task},
}; // Replace `my_project` with the name of your crate

#[test]
fn test_proj_create() {
    // File info setup
    let file_title = "test_project";
    let file_path = format!("tests/.{}", file_title);
    let file_path = std::path::Path::new(&file_path);

    // Check that file doesn't already exist
    if file_path.exists() {
        fs::remove_file(file_path).expect("Failed to delete file prior to running tests.");
    }

    // Test that file is created
    assert!(project_utils::create_project(file_title, file_path).is_ok());
    assert!(file_path.exists());

    // Test that trying to create an existing file results in error
    assert!(project_utils::create_project(file_title, file_path).is_err());
    assert!(file_path.exists());

    // Clean up
    std::fs::remove_file(file_path).expect("Failed to delete file after running tests");
}

#[test]
fn test_proj_io() {
    // Setup file info
    let file_title = "write_test.JSON";
    let file_path = format!("tests/.{}", file_title);
    let file_path = Path::new(&file_path);

    // Setup project info
    let mut proj = Project::new("Test".to_string());
    let sess = Session::new(Utc::now());
    let mut task = Task::new("Testing".to_string());
    task.sessions.push(sess);
    proj.tasks.push(task);

    // Ensure that the file doesn't exist
    if file_path.exists() {
        fs::remove_file(file_path).expect("Failed to delete file prior to running tests.");
    }

    // Attempt to write and read to/from non-existent file
    assert!(write_project(file_path, &proj).is_err());
    assert!(read_project(file_path).is_err());

    // Create empty test file
    File::create(file_path).expect("Failed to create file before running test");
    assert!(file_path.exists());

    // Write to test file
    assert!(write_project(file_path, &proj).is_ok());

    // Read from test file
    let test_proj = read_project(file_path).expect("Failed to read propject into ");

    // Compare projects
    assert!(test_proj == proj);

    remove_file(file_path).expect("Should not fail");
}
