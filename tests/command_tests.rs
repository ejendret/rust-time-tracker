use rust_time_tracker::command_utils::process_args;

#[test]
fn test_process() {
    let args: Vec<String> = vec!["tracker", "project", "create", "test"]
        .into_iter()
        .map(String::from)
        .collect();

    process_args(&args);
}
