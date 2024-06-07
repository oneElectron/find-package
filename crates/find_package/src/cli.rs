use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,

    /// Print more information
    #[arg(short, long)]
    pub verbose: bool,

    /// Silence output
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Subcommand, Clone, Debug)]
pub(crate) enum CliCommand {
    /// Search for packages
    Search {
        /// Comma separated list of repositories to search through
        #[arg(short, long)]
        repos: Option<String>,

        /// Comma separated list of file types to search for
        #[arg(short, long)]
        types: Option<String>,

        /// search query
        query: String,
    },
}
