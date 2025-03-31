use crate::levels;
use crate::outputs::Outputs;
use std::process::exit;

pub fn log(level: levels::LogLevel, msg: String, out: Outputs) {
    match level {
        levels::LogLevel::Warn => {
            println!("WARN: {}", msg);
        }

        levels::LogLevel::Info => {
            println!("INFO: {}", msg)
        }

        levels::LogLevel::Err => {
            if checkout(out) == 0 {
                println!("ERR: {}", msg);
            }
            eprintln!("ERR: {}", msg)
        }

        levels::LogLevel::Fatal => {
            if checkout(out) == 0 {
                println!("Fatal: {}", msg);
                exit(1);
            }
            eprintln!("Fatal: {}", msg);
        }
    }
}

fn checkout(out: Outputs) -> i32 {
    match out {
        Outputs::Allstdout => return 0,
        Outputs::Errtostderr => return 1,
    }
}
