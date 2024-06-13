// tests/proj_tests.rs
use rust_time_tracker::project_utils; // Replace `my_project` with the name of your crate

#[test]
fn test_proj_create() {
    // File info setup
    let file_title = "test_project";
    let file_path = format!(".{}", file_title);
    let file_path = std::path::Path::new(&file_path);

    // Check that file doesn't already exist
    if file_path.exists() {
        std::fs::remove_file(file_path).expect("Failed to delete file prior to running tests.");
    }

    // Test that file is created
    assert!(project_utils::proj_create(file_title).is_ok());
    assert!(file_path.exists());
    
    // Test that trying to create an existing file results in error
    assert!(project_utils::proj_create(file_title).is_err());
    assert!(file_path.exists());

    // Clean up
    std::fs::remove_file(file_path).expect("Failed to delete file after running tests");
}
