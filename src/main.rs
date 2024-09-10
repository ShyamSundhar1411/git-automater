mod operations;
mod helpers;
mod license;
mod commits;
mod branches;
mod gitignore;
use simplelog::*;
use std::fs::File;
use log::info;
fn main() {
    let log_file = File::create("logs").unwrap();
    let _ = CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    );
    info!("Starting git-automater");
    loop{
        operations::prompt();
    }
}
