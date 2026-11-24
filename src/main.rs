use std::{println, process::ExitCode};

mod entry;
mod vault;
mod cli;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match cli::parse_command(&args) {
        Ok(command) => {
            println!("Parsed command: {command:?}");
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("Error in parsing command: {err}");
            ExitCode::FAILURE
        }
    }
}
