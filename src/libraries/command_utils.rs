#[derive(PartialEq, Eq)]
pub enum Command {
    Invalid,
    Help,
    ProjectView,
    ProjectList,
    ProjecCreate,
    ProjectDelete,
    ProjectCheckout,
}

pub fn process_args(args: &Vec<String>) -> Command {
    match args[0].as_str() {
        "help" => Command::Help,
        "project" => {
            match args.len() {
                2 => {
                    match args[1].as_str() {
                        "view" => Command::ProjectView,
                        "list" => Command::ProjectList,
                        _ => Command::Invalid
                    }
                }
                3 => {
                    match args[1].as_str() {
                        "create" => Command::ProjecCreate,
                        "delete" => Command::ProjectDelete,
                        "checkout" => Command::ProjectCheckout,
                        _ => Command::Invalid
                    }
                }
                _ => Command::Invalid
            }
        }
        "task" => {
           Command::Invalid
        }
        _ => Command::Invalid
    }
}
