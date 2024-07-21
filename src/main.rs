mod cli;
use clap::Parser;
use cli::{Cli, CliCommand};

fn main() {
    let args = Cli::parse();

    match args.command {
        CliCommand::Search { .. } => {}

        #[allow(unreachable_patterns)]
        _ => {
            std::process::exit(0);
        }
    }
}
