use std::{
    fs::{remove_dir_all, remove_file},
    io::Cursor,
    path::Path,
};

use rust_time_tracker::config_utils::{build_config, config_present, get_confirmation, Config};

#[test]
fn test_valid_get_set() {
    // Create a config and populate the fields
    let mut config = Config::new(None, None);

    for i in 0..10 {
        let title = format!("test project {}", i);
        config.add_project(title.clone());
        config
            .set_current_proj(title.clone())
            .expect("Should not fail.");
        assert!(config.get_projects()[i] == title && config.get_current_proj() == title);
    }

    for i in 10..0 {
        let title = format!("test project {}", i);
        config
            .set_current_proj(title.clone())
            .expect("Should not fail.");
        config.remove_project(&title);
        assert!(config.get_current_proj() == "none");
        assert!(config.get_projects().contains(&title) == false);
    }
}

#[test]
fn test_invalid_get_set() {
    // Create a config and populate the fields
    let mut config = Config::new(None, None);
    assert!(config.set_current_proj("test".to_string()).is_err());
    config.add_project("test".to_string());
    config.add_project("test".to_string());
}

#[test]
fn test_to_from() {
    // Create a config and populate the fields
    let mut before_config = Config::new(None, None);

    for i in 0..10 {
        before_config.add_project(format!("test project {}", i));
    }

    before_config
        .set_current_proj("test project 5".to_string())
        .expect("Failed to set current project.");

    let path = "tests/files/config.JSON";

    // Write the config to a file
    before_config
        .to_file(path)
        .expect("Failed to write to file.");

    // Read from the file into a new config and compare
    let after_config = Config::from_file(path).expect("Failed to read from file.");
    assert!(before_config == after_config);

    remove_file(path).expect("Should not fail");
}

#[test]
fn test_get_confirmation() {
    assert!(get_confirmation("This is a test query", Cursor::new(b"negative")).is_err());
    assert!(get_confirmation("This is a test query", Cursor::new(b"yes")).is_ok());
}

#[test]
fn test_build_check_config() {
    let dir = "tests/files/check_config";
    let dir_path = Path::new(dir);
    let file = "tests/files/check_config/config.JSON";
    let file_path = Path::new(file);

    // If testing directory exists, remove
    if dir_path.exists() {
        remove_dir_all(dir_path).expect("Should not fail");
    }

    assert!(!file_path.exists() && !dir_path.exists());

    assert!(config_present(dir) == false);

    // Create program dir and config file
    build_config(dir).expect("Should not fail");
    assert!(dir_path.exists() && file_path.exists());
    assert!(config_present(dir) == true);

    remove_dir_all(dir_path).expect("Should not fail");
}
