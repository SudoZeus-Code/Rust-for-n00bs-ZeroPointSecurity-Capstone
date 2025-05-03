use std::fs::File;
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::Write;

// for time delay
use std::thread;
use std::time::Duration;


pub fn filechecks() {

    exists()

}

fn exists() {
    
    let output = Command::new("ls").stdout(Stdio::piped()).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    //dbg!(&stdout);
    if !stdout.contains("library.txt") {
        Command::new("touch").arg("library.txt").spawn().expect("Failed to create library file.");
    }

}

pub fn write(i: String) {

    
    // add new line to string 
    
    // append our items to our file
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("library.txt")
        .expect("cannot open file");

    data_file.write(i.as_bytes()).expect("Failed to write");

    println!("Item added!");

    let tic = Duration::from_millis(2000);


}