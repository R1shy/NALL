use std::{fs::File, path::Path};
use crate::levels::LogLevel;


pub fn jlog(path: String, msg: String, level: LogLevel) {
    let tpath = Path::new(&path);

    match tpath.exists() {
        true => {
            logoldfile(path,msg,level);
        }

        false => {
            lognewfile(path, msg,level);
        }
    } 
}


fn logoldfile(path: String, msg: String, level: LogLevel) {
    todo!()
}

fn lognewfile(path: String, msg: String, level: LogLevel) {

    let tpath = Path::new(&path);
    let display = tpath.display();
     let mut file = match File::create(&tpath) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
