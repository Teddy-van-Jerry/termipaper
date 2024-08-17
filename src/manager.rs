use super::options::Cli;
use super::options::Config;
use super::options::PaperDir;
use crate::options::Commands;
use crate::options::ConfigDatabase;

#[derive(Debug, Clone)]
pub struct Manager {
    pub args: Cli,
    pub dir: String,
    pub config: Config,
}
impl PaperDir for Manager {}

impl Manager {
    pub fn new() -> Self {
        let config = Config::from_file();
        let args = Cli::get_args();
        let dir = match args.clone().dir {
            Some(dir) => dir.clone(),
            None => match &config.activated {
                Some(dir) => dir.clone(),
                None => Self::_default_dir_str(),
            },
        };
        Self { args, dir, config }
    }

    /// Check if two directories are the same
    ///
    /// A directory is considered the same if the absolute path of both directories are the same.
    /// This function asserts both paths are valid and exist.
    fn _is_same_dir(dir1: &String, dir2: &String) -> bool {
        std::fs::canonicalize(dir1).unwrap() == std::fs::canonicalize(dir2).unwrap()
    }

    fn _is_dir_existent(dir: &String) -> bool {
        std::fs::metadata(dir).is_ok()
    }

    fn _ck_dir(dir: &String) -> Result<(), ()> {
        if !Self::_is_dir_existent(dir) {
            eprintln!("Error: Directory does not exist: {}", dir);
            Err(())
        } else {
            Ok(())
        }
    }

    /// TermiPaper Command: init
    pub fn cmd_init(&mut self) -> Result<(), ()> {
        // 1. Determine which directory the user wants to initialize
        let dir_to_init = match &self.args.cmd {
            Commands::Init(args) => match &args.dir {
                Some(dir) => {
                    if !Self::_is_dir_existent(dir) {
                        // create directory if not existent
                        match std::fs::create_dir_all(dir) {
                            Ok(_) => {},
                            Err(_) => {
                                eprintln!("Error: Cannot create directory for initialization: {}", dir);
                                return Err(());
                            }
                        }
                    }
                    dir.clone()
                }
                None => self.dir.clone(),
            },
            _ => {
                assert!(
                    false,
                    "Internal Error: This function should only be called with the 'init' command."
                );
                "".to_string()
            }
        };
        // 2. Check if the directory is the same as the activated directory
        if Self::_is_same_dir(&dir_to_init, &self.dir) {
            println!(
                "Info: The current database is already activated: {}",
                dir_to_init
            );
            return Ok(());
        }
        // 3. Check if the directory is already initialized from the databases
        let mut is_initialized = false;
        if let Some(databases) = &self.config.databases {
            for database in databases {
                if Self::_is_same_dir(&dir_to_init, &database.0) {
                    is_initialized = true;
                    break;
                }
            }
        }
        // 4. Initialize the directory (if not)
        if !is_initialized {
            let mut databases = match &self.config.databases {
                Some(databases) => databases.clone(),
                None => std::collections::HashMap::new(),
            };
            databases.insert(dir_to_init.clone(), ConfigDatabase::new());
            self.config.databases = Some(databases);
            self.config.to_file();
            println!("Info: Initialized database: {}", dir_to_init);
        } else {
            println!("Info: The database is already initialized: {}", dir_to_init);
        }
        Ok(())
    }
}
