use crate::levels::LogLevel;
use ansi_term::Colour::*;
use crate::levels;
use std::process::exit;

pub fn clog(level: levels::LogLevel, msg: String) {
    
    let mut warnc = Yellow.paint("WARN").to_string();
    let mut infoc = Blue.paint("INFO").to_string();
    let mut Errc = Red.paint("ERR").to_string();
    let mut Fatalc = Green.paint("Fatal").to_string();

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
