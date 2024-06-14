// src/main.rs
// My libraries
// use rust_time_tracker::project_utils;
// use rust_time_tracker::task_utils;
use rust_time_tracker::config_utils;

// Outside libraries
// use std::env;

fn main() {
    // Retrieve arguments
    // let args: Vec<String> = env::args().collect();

    // let first = &args[1];

    // if let Err(e) = project_utils::proj_create(first) {
    //     println!("[main] error creating project: {}", e);
    // }

    match config_utils::get_confirmation("testing?") {
        Ok(()) => println!("Confirmation"),
        Err(why) => println!("[main] failed to get confirmation: {}", why)

    };
}
