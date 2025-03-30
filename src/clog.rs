use crate::levels;
use crate::levels::LogLevel;
use std::process::exit;

pub fn clog(level: levels::LogLevel, msg: String) {
    let mut Warnc = "\x1b[222mWARN\x1b[0m";
    let mut Infoc = "\x1b[63mINFO\x1b[0m";
    let mut Errc = "\x1b[1mERR\x1b[0m";
    let mut Fatalc = "\x1b[52mFATAL\x1b[0m";

    match level {
        levels::LogLevel::Warn => {
            println!("{}: {}", warnc, msg);
        }

        levels::LogLevel::Info => {
            println!("{}: {}", infoc, msg)
        }

        levels::LogLevel::Err => {
            println!("{}: {}", Errc, msg);
        }

        levels::LogLevel::Fatal => {
            println!("{}: {}", Fatalc, msg);
            exit(1);
        }
    }
}
