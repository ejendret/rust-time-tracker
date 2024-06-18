use std::{fs::File, io::BufWriter, path::Path};

// src/main.rs
// My libraries
// use rust_time_tracker::project_utils;
use rust_time_tracker::{project_utils::{create_project, retrieve_project, write_project, Project}, task_utils::{Session, Task}};
// Outside libraries
use chrono::Utc;
// use std::env;

fn main() {
    let mut proj = Project::new("Test".to_string());
    let sess = Session::new(Utc::now());
    let mut task = Task::new("Testing".to_string());
    task.sessions.push(sess);
    proj.tasks.push(task);
    println!("Ermm proj? {:?}", proj);
    let title = "testing";
    let path = format!("/home/ejendret/.tracker/{}.txt", title);
    let path = Path::new(&path);
    match create_project(title, path) {
        Ok(status) => println!("{}", status),
        Err(status) => println!("{}", status),
    }
    println!("Before: {:?}", retrieve_project(path));
    match write_project(path, &proj) {
        Ok(status) => println!("{}", status),
        Err(status) => println!("{}", status),
    }
    println!("After: {:?}", retrieve_project(path));
    // println!("{:?}", project);
}
