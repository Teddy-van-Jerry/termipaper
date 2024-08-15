use clap::{Args, Parser, Subcommand};

/// A terminal-based academic paper manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    cmd: Commands,
    /// Whether to print verbose output
    #[arg(short, long, default_value = "false", global = true)]
    verbose: bool,
}

impl Cli {
    pub fn dbg_args() {
        let args = Self::parse();
        dbg!(args);
    }
}

#[derive(Args, Clone, Debug)]
struct CommandAddArgs {
    #[arg(index = 1)]
    id: String,
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new paper to the database
    Add(CommandAddArgs),
}
