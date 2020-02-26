extern crate log;
extern crate simplelog;

use simplelog::*;
use std::boxed::Box;
use std::fs::File;

pub fn logger(logfile: &str) -> Box<CombinedLogger> {
    let new_logger = CombinedLogger::new(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(logfile).unwrap(),
        ),
    ]);
    return new_logger;
}
