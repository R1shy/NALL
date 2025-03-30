use crate::levels::LogLevel;
use std::{fs::File, io::Write, path::Path};

pub fn flog(path: String, msg: String, level: LogLevel) {
    let tpath = Path::new(&path);

    match tpath.exists() {
        true => {
            logoldfile(path, msg, level);
        }

        false => {
            lognewfile(path, msg, level);
        }
    }
}

fn logoldfile(path: String, msg: String, level: LogLevel) {
    let fmsg: String;
    let tpath = Path::new(&path);
    let display = tpath.display();
    let mut file = match File::open(&tpath) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match level {
        LogLevel::Info => fmsg = format!("INFO: {}", msg),
        LogLevel::Warn => fmsg = format!("WARN: {}", msg),
        LogLevel::Err => fmsg = format!("ERR: {}", msg),
        LogLevel::Fatal => fmsg = format!("FATAL: {}", msg),
    }

    match file.write_all(fmsg.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn lognewfile(path: String, msg: String, level: LogLevel) {
    let fmsg: String;
    let tpath = Path::new(&path);
    let display = tpath.display();
    let mut file = match File::create(&tpath) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match level {
        LogLevel::Info => fmsg = format!("INFO: {}", msg),
        LogLevel::Warn => fmsg = format!("WARN: {}", msg),
        LogLevel::Err => fmsg = format!("ERR: {}", msg),
        LogLevel::Fatal => fmsg = format!("FATAL: {}", msg),
    }

    match file.write_all(fmsg.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
