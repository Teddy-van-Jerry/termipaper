extern crate directories;
use super::options::Cli;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug)]
pub struct Manager {
    pub dir: String,
    pub verbose: bool,
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

impl PaperDir for Manager { }

impl Manager {
    pub fn new() -> Self {
        let args = Cli::get_args();
        let dir = match args.dir {
            Some(dir) => dir,
            None => Self::_default_dir_str(),
        };
        Self {
            dir,
            verbose: args.verbose,
        }
    }
}
