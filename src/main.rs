mod cli;
use cli::{Cli, CliCommand};
use clap::Parser;

mod macros;

fn main() {
    let args = Cli::parse();


    match args.command {
        CliCommand::Search { .. } => {

        }

        _ => {
            std::process::exit(0);
        }
    }
}
