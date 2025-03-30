use crate::levels;
use std::process::exit;

pub fn clog(level: levels::LogLevel, msg: String) {
    let warnc = "\x1b[38;5;222mWARN\x1b[0m";
    let infoc = "\x1b[38;5;63mINFO\x1b[0m";
    let errc = "\x1b[38;5;1mERR\x1b[0m";
    let fatalc = "\x1b[38;5;52mFATAL\x1b[0m";

    match level {
        levels::LogLevel::Warn => {
            println!("{}: {}", warnc, msg);
        }

        levels::LogLevel::Info => {
            println!("{}: {}", infoc, msg)
        }

        levels::LogLevel::Err => {
            println!("{}: {}", errc, msg);
        }

        levels::LogLevel::Fatal => {
            println!("{}: {}", fatalc, msg);
            exit(1);
        }
    }
}
