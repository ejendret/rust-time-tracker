use std::path::Path;

// src/main.rs
// My libraries
// use rust_time_tracker::project_utils;
use rust_time_tracker::{
    project_utils::{create_project, read_project, write_project, Project},
    task_utils::{Session, Task},
};
// Outside libraries
use chrono::Utc;
// use std::env;

fn main() {
    let mut proj = Project::new("Test".to_string());
    let sess = Session::new(Utc::now());
    let mut task = Task::new("Testing".to_string());
    task.sessions.push(sess);
    proj.tasks.push(task);
    let title = "testing";
    let path = format!("/home/ejendret/.tracker/{}.txt", title);
    let path = Path::new(&path);
    if let Err(e) = create_project(title, path) {
        println!("{}", e);
    }

    println!("Before: {:?}", read_project(path));
    match write_project(path, &proj) {
        Ok(status) => println!("{}", status),
        Err(status) => println!("{}", status),
    }
    println!("After: {:?}", read_project(path));
}
