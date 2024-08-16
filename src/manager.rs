use super::options::Cli;
use super::options::PaperDir;

#[derive(Debug)]
pub struct Manager {
    pub dir: String,
    pub verbose: bool,
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
