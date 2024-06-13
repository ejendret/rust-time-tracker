use std::env;
// use std::process::exit;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

// Will create a hidden file containing project data in the current directory
fn proj_create(title: String) {
    // Get projedt path
    let proj_path = format!(".{}", title);
    let proj_path = Path::new(&proj_path);

    // Create a file for the project
    let mut proj_file = match File::create_new(&proj_path) {
        Err(why) => panic!("Couldn't create {}: {}", proj_path.display(), why),
        Ok(proj_file) => proj_file,
    }; 

    // Write title to file
    match proj_file.write_all(title.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", proj_path.display(), why),
        Ok(()) => println!("successfully wrote {} to {}", title, proj_path.display())
    };
}

fn main() {
    // Retrieve arguments
    let args: Vec<String> = env::args().collect();

    let first = &args[1];

    proj_create(first.to_string());
}
