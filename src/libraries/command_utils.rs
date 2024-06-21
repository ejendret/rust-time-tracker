pub fn process_args(args: &Vec<String>) {
    println!("Num args: {}", args.len());
    match args.len() {
        // One argument
        2 => {}
        _ => {
            println!("Please enter a valid command\nhint: tracker help");
        }
    }
}
