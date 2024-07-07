use rust_time_tracker::command_utils::{parse_args, Command};

#[test]
fn test_process() {
    let args_list: Vec<Vec<String>> = vec![vec![]];
   
    let args: Vec<String> = vec!["project", "create", "test"]
        .into_iter()
        .map(String::from)
        .collect();

    assert!(parse_args(&args) == Command::ProjectCreate);

    let args: Vec<String> = vec!["help"]
    .into_iter()
    .map(String::from)
    .collect();

    assert!(parse_args(&args) == Command::Help);
}
