extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

pub fn logger(logfile: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(logfile).unwrap(),
        ),
    ]).unwrap();
}
