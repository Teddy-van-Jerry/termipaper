extern crate directories;
extern crate serde;
extern crate chrono;
use std::path::PathBuf;
use std::collections::HashMap;
use clap::{Args, Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

/// A terminal-based academic paper manager
#[derive(Debug, Clone, Parser)]
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
    pub dir: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandAddArgs {
    #[arg(index = 1)]
    pub id: String,
    #[arg(short, long)]
    pub file: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandEditArgs {
    #[arg(index = 1)]
    pub id: String,
    #[arg(short, long)]
    pub file: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandInfoArgs {
}

#[derive(Args, Clone, Debug)]
pub struct CommandInitArgs {
    #[arg(index = 1)]
    pub dir: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct CommandListArgs {
}

#[derive(Args, Clone, Debug)]
pub struct CommandOpenArgs {
    #[arg(index = 1)]
    pub id: String,
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

#[derive(Subcommand, Debug, Clone)]
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
    /// Open a paper
    Open(CommandOpenArgs),
    /// Get information of the database
    Info(CommandInfoArgs),
    /// Initialize the paper database
    Init(CommandInitArgs),
    /// Search for papers in the database
    Search(CommandSearchArgs),
    /// Show a paper's details
    Show(CommandShowArgs),
}

pub trait PaperDir {
    fn _project_dirs() -> ProjectDirs {
        if let Some(project_dirs) = ProjectDirs::from("org", "wqzhao", "termipaper") {
            project_dirs
        } else {
            panic!("Cannot find the termipaper project directories");
        }
    }
    
    fn _config_dir() -> PathBuf {
        Self::_project_dirs().config_dir().to_path_buf()
    }

    fn _config_dir_str() -> String {
        Self::_config_dir().to_str().unwrap().to_string()
    }

    fn _config_parent_dir() -> PathBuf {
        Self::_config_dir().parent().unwrap().to_path_buf()
    }

    fn _data_dir() -> PathBuf {
        Self::_project_dirs().data_dir().to_path_buf()
    }

    fn _data_dir_str() -> String {
        Self::_data_dir().to_str().unwrap().to_string()
    }

    /// Default directory for termipaper papers (<data_dir>/papers)
    fn _default_dir() -> PathBuf {
        Self::_data_dir().join("papers").to_path_buf()
    }

    fn _default_dir_str() -> String {
        Self::_default_dir().to_str().unwrap().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigDatabase {
    /// format of data created is "YYYY-MM-DD"
    pub date_created: String,
}

impl ConfigDatabase {
    pub fn new() -> Self {
        Self {
            date_created: chrono::Local::now().format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigOwner {
    pub name: String,
    pub email: Option<String>,
    pub affiliation: Option<String>,
    pub link: Option<String>,
}

pub type ConfigDatabases = HashMap<String, ConfigDatabase>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The databases of papers
    pub databases: Option<ConfigDatabases>,
    /// owner of the config file
    pub owner: Option<ConfigOwner>,
    /// The activated paper directory
    pub activated: Option<String>,
}

impl PaperDir for Config { }

impl Config {
    fn _config_file() -> PathBuf {
        let config_dir = Self::_config_dir();
        config_dir.join("config.yml")
    }

    pub fn new() -> Self {
        Self {
            databases: None,
            owner: None,
            activated: None,
        }
    }

    /// Load the config from the config file
    pub fn from_file() -> Self {
        let config_file = Self::_config_file();
        if config_file.exists() {
            let config_str = std::fs::read_to_string(config_file).unwrap();
            match serde_yaml::from_str(&config_str) {
                Ok(config) => config,
                Err(_) => {
                    eprintln!("Error: Cannot parse the config file at '{}'.", Self::_config_dir_str());
                    Self::new()
                }
            }
        } else {
            Self::new()
        }
    }

    /// Save the config to the config file
    /// 
    /// It will overwrite the existing config file, so be careful.
    pub fn to_file(&self) {
        let config_file = Self::_config_file();
        let config_str = serde_yaml::to_string(&self).unwrap();
        // needs to create the config directory first
        std::fs::create_dir_all(Self::_config_dir()).unwrap();
        std::fs::write(config_file, config_str).unwrap();
    }
}

/*
// DANGEROUS test: it can overwrite the config file
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        println!("Test Config at {}", Config::_config_dir_str());
        let config = Config::new();
        config.to_file();
        let config = Config::from_file();
        assert_eq!(config, Config::new());
    }
}
*/
