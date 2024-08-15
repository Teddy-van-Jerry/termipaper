use termipaper::Cli;
use termipaper::Manager;

fn main() {
    Cli::dbg_args();
    let manager = Manager::new();
    dbg!(manager);
}
