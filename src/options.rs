use clap::{Args, Parser, Subcommand};

/// A terminal-based academic paper manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    pub cmd: Commands,
    /// Activated paper directory
    #[arg(short, long, global = true)]
    pub dir: Option<String>,
    /// Whether to print verbose output
    #[arg(short, long, default_value = "false", global = true)]
    pub verbose: bool,
}

impl Cli {
    pub fn get_args() -> Self {
        Self::parse()
    }

    pub fn dbg_args() {
        let args = Self::parse();
        dbg!(args);
    }
}

#[derive(Args, Clone, Debug)]
pub struct CommandActivateArgs {
    #[arg(index = 1)]
    dir: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandAddArgs {
    #[arg(index = 1)]
    id: String,
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandEditArgs {
    #[arg(index = 1)]
    id: String,
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandInitArgs {
    #[arg(index = 1)]
    dir: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandListArgs {
}

#[derive(Args, Clone, Debug)]
pub struct CommandOpenArgs {
    #[arg(index = 1)]
    id: String,
}

#[derive(Args, Clone, Debug)]
pub struct CommandSearchArgs {
}

#[derive(Args, Clone, Debug)]
pub struct CommandShowArgs {
}

#[derive(Args, Clone, Debug)]
pub struct CommandRemoveArgs {
    #[arg(index = 1)]
    id: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Activate a paper directory
    Activate(CommandActivateArgs),
    /// Add a new paper to the database
    Add(CommandAddArgs),
    /// Edit a paper in the database
    Edit(CommandEditArgs),
    /// List papers in the database
    List(CommandListArgs),
    /// Remove a paper from the database
    Remove(CommandRemoveArgs),
    /// Initialize the paper database
    Init(CommandInitArgs),
    /// Search for papers in the database
    Search(CommandSearchArgs),
    /// Show a paper's details
    Show(CommandShowArgs),
}
