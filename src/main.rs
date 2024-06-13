// src/main.rs
// My libraries
use rust_time_tracker::project_utils;
// use rust_time_tracker::task_utils;
// Outside libraries
use std::env;


fn main() {
    // Retrieve arguments
    let args: Vec<String> = env::args().collect();

    let first = &args[1];

    match project_utils::proj_create(first) {
        Ok(()) => println!("Succesfuly created project"),
        Err(e) => println!("[main] error creating project: {}", e),
    };
}
