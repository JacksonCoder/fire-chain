use firechain::error::fatal_error;
pub enum Command {
    Server,
    Unknown
}

pub struct Args {
    pub ledger_file_path: String,
    pub command: Command
}

pub fn argparse(args: Vec<String>) -> Args {
    if args.len() < 2 {
        fatal_error("Not enough arguments were passed to the program. At least one argument (the ledger file path) is required",-1);
    }
    Args {
        ledger_file_path: args[1].clone(),
        command: {
            Command::Unknown
        }
    }
}