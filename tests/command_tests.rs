use rust_time_tracker::command_utils::{process_args, Command};

#[test]
fn test_process() {
    let args: Vec<String> = vec!["tracker", "project", "create", "test"]
        .into_iter()
        .map(String::from)
        .collect();

    assert!(process_args(&args) == Command::Invalid);

    let args: Vec<String> = vec!["tracker", "help"]
    .into_iter()
    .map(String::from)
    .collect();

    assert!(process_args(&args) == Command::Help);
}
