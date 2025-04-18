use crate::levels;
use std::process::exit;

pub fn clog(level: levels::LogLevel, msg: String, color: Option<i32>) {
    let col: i32 = color.unwrap_or(20);
    let warnc = format!("\x1b[38;5;{}mWARN\x1b[0m", col);
    let infoc = format!("\x1b[38;5;{}mINFO\x1b[0m", col);
    let errc = format!("\x1b[38;5;{}mERR\x1b[0m", col);
    let fatalc = format!("\x1b[38;5;{}mFATAL\x1b[0m", col);

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
