use termipaper::Cli;
use termipaper::Manager;

fn main() {
    Cli::dbg_args();
    let manager = Manager::new();
    dbg!(&manager);
    if let Err(_) = manager.run() {
        std::process::exit(1);
    }
}
