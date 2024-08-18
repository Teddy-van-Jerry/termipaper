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

    pub fn run(&self) -> Result<(), ()> {
        match &self.args.cmd {
            Commands::Activate(_) => self.cmd_activate(),
            // Commands::Add(_) => self.cmd_add(),
            Commands::Config(_) => self.cmd_config(),
            // Commands::Edit(_) => self.cmd_edit(),
            Commands::Info(_) => self.cmd_info(),
            Commands::Init(_) => self.cmd_init(),
            // Commands::List(_) => self.cmd_list(),
            // Commands::Open(_) => self.cmd_open(),
            // Commands::Search(_) => self.cmd_search(),
            _ => {
                unimplemented!("Command not implemented: {:?}", self.args.cmd);
            }
        }
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

    fn _get_dir(&self) -> Result<String, ()> {
        let dir = match &self.args.cmd {
            Commands::Activate(args) => match &args.dir {
                Some(dir) => dir.clone(),
                None => self.dir.clone(),
            },
            Commands::Init(args) => match &args.dir {
                Some(dir) => dir.clone(),
                None => self.dir.clone(),
            },
            _ => {
                assert!(
                    false,
                    "Internal Error: This function should only be called with a command containing the dir field."
                );
                "".to_string()
            }
        };
        if !Self::_is_dir_existent(&dir) {
            // create directory if not existent
            match std::fs::create_dir_all(&dir) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error: Cannot create directory: {}", &dir);
                    return Err(());
                }
            }
        }
        Ok(std::fs::canonicalize(&dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string())
    }

    pub fn _is_initialized(&self, dir: &String) -> bool {
        if let Some(databases) = &self.config.databases {
            for database in databases {
                if Self::_is_same_dir(&dir, &database.0) {
                    return true;
                }
            }
        }
        false
    }

    pub fn cmd_activate(&self) -> Result<(), ()> {
        // 1. Determine which directory the user wants to activate
        let dir_to_activate = self._get_dir()?;
        if let Some(activated) = self.config.activated.clone() {
            if Self::_is_same_dir(&dir_to_activate, &activated) {
                println!(
                    "Info: The current database is already activated: {}",
                    dir_to_activate
                );
                return Ok(());
            }
        }
        // 2. Check if the directory is initialized
        let is_initialized = self._is_initialized(&dir_to_activate);
        if !is_initialized {
            eprintln!(
                "Error: The database is not initialized: {}",
                dir_to_activate
            );
            return Err(());
        }
        // 3. Activate the directory
        let mut new_config = self.config.clone();
        new_config.activated = Some(dir_to_activate.clone());
        new_config.to_file();
        println!("Info: Activated database: {}", dir_to_activate);
        Ok(())
    }

    pub fn cmd_info(&self) -> Result<(), ()> {
        // get the activated database
        let activated = match &self.config.activated {
            Some(activated) => activated.clone(),
            None => {
                println!("Info: No database is activated.");
                return Ok(());
            }
        };
        // check if the activated database is valid
        if Self::_is_dir_existent(&activated) {
            println!("Info: Activated database: {}", activated);
        } else {
            eprintln!(
                "Error: The activated database does not exist: {}",
                activated
            );
            return Err(());
        }
        Ok(())
    }

    pub fn cmd_config(&self) -> Result<(), ()> {
        Ok(())
    }

    /// TermiPaper Command: init
    ///
    /// TODO: check whether the database is healthy (for example non-existent directories)
    pub fn cmd_init(&self) -> Result<(), ()> {
        // 1. Determine which directory the user wants to initialize
        let dir_to_init = self._get_dir()?;
        // 2. Check if the directory is the same as the activated directory
        if let Some(activated) = &self.config.activated {
            if Self::_is_same_dir(&dir_to_init, &activated) {
                println!(
                    "Info: The current database is already activated: {}",
                    dir_to_init
                );
                return Ok(());
            }
        };
        // 3. Check if the directory is already initialized from the databases
        let is_initialized = self._is_initialized(&dir_to_init);
        // 4. Initialize the directory (if not)
        if !is_initialized {
            let mut databases = match &self.config.databases {
                Some(databases) => databases.clone(),
                None => std::collections::HashMap::new(),
            };
            databases.insert(dir_to_init.clone(), ConfigDatabase::new());
            let mut new_config = self.config.clone();
            new_config.databases = Some(databases);
            new_config.activated = Some(dir_to_init.clone()); // activate the new database
            new_config.to_file();
            println!("Info: Initialized database: {}", dir_to_init);
        } else {
            println!("Info: The database is already initialized: {}", dir_to_init);
        }
        Ok(())
    }
}
