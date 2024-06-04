use std::fs::OpenOptions;
use std::io::Write;

pub fn log(out: &str) {
    let mut file = OpenOptions::new().append(true).open("debug.log").unwrap();

    // Write the content to the file
    file.write_all(out.as_bytes()).unwrap();

    //println!("Debug data has been written to log.");
}
