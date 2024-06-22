use rust_time_tracker::command_utils::{process_args, Command};

#[test]
fn test_process() {
    let args: Vec<String> = vec!["project", "create", "test"]
        .into_iter()
        .map(String::from)
        .collect();

    assert!(process_args(&args) == Command::ProjectCreate);

    let args: Vec<String> = vec!["help"]
    .into_iter()
    .map(String::from)
    .collect();

    assert!(process_args(&args) == Command::Help);
}
