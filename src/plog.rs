use crate::levels;
use std::process::exit;

pub fn log(level: levels::LogLevel, msg: String) {
    match level {
        levels::LogLevel::Warn => {
            println!("WARN: {}", msg);
        }

        levels::LogLevel::Info => {
            println!("INFO: {}", msg)
        }

        levels::LogLevel::Err => {
            println!("ERR: {}", msg);
        }

        levels::LogLevel::Fatal => {
            println!("Fatal: {}", msg);
            exit(1);
        }
    }
}
