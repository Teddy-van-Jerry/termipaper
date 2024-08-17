use super::options::Cli;
use super::options::PaperDir;
use super::options::Config;

#[derive(Debug, Clone)]
pub struct Manager {
    pub dir: String,
    pub verbose: bool,
    pub config: Config,
}
impl PaperDir for Manager { }

impl Manager {
    pub fn new() -> Self {
        let config = Config::from_file();
        let args = Cli::get_args();
        let dir = match args.dir {
            Some(dir) => dir,
            None => {
                match &config.activated {
                    Some(dir) => dir.clone(),
                    None => Self::_default_dir_str(),
                }
            }
        };
        Self {
            dir,
            verbose: args.verbose,
            config,
        }
    }
}
