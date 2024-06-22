#[derive(PartialEq, Eq)]
pub enum Command {
    Invalid,
    Help,
    ProjecCreate,
    ProjctDelete,
    ProjectView,
}

pub fn process_args(args: &Vec<String>) -> Command {
    match args.len() {
        // One argument
        2 => {
            match args[1].as_str() {
                "help" => Command::Help,
                _ => Command::Invalid
            }
        }
        // Two arguments
        _ => Command::Invalid
        }
}
