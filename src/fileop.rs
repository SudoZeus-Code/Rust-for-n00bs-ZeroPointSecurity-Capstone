use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
//use std::fs::OpenOptions;
//use std::io::{Write, Read};
use std::io::{stdin, stdout, Read, Write};

// for time delay
use std::thread;
use std::time::Duration;

//json reading
use serde::{Serialize,Deserialize};
//use serde_json::json;
use serde_json::{from_reader, to_writer_pretty};
use std::path::Path;
use std::error::Error;
//use std::fs::File
use std::io::{BufReader,BufWriter};
//use std::path::Path;


// json testing
// #[derive(Debug, Deserialize)]
// struct Todoitem {
// 	title: String,
// 	body: String,
// 	priority: String,
// 	status: String,
// 	duedate: String,
// }

use crate::models::Todoitem;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn exists() {
    
    let output = Command::new("ls").stdout(Stdio::piped()).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    //dbg!(&stdout);
    if !stdout.contains("library.json") {
        Command::new("touch").arg("library.json").spawn().expect("Failed to create library file.");
    }

}

pub fn write(i: String) {





    // start of new
    let ser_item: Todoitem = serde_json::from_str(&i).expect("bad input JSON");
    let path = "library.json";
    let file = OpenOptions::new().read(true).open(path);

    let mut items: Vec<Todoitem> = match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    };

    items.push(ser_item);

    //rewrite with updated array
    let file = File::create(path).expect("Cant create file");
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, &items).expect("Failed to write.");
    //end of new

    //Writes new items to our file.
    
    // add new line to string so our file is seperated by lines
    //let inewline = format!("{}\n", i);
    
    // // append our items to our file
    // let mut data_file = OpenOptions::new()
    //     .append(true)
    //     .open("library.json")
    //     .expect("cannot open file");

    // //data_file.write(inewline.as_bytes()).expect("Failed to write");
    // data_file.write(i.as_bytes()).expect("Failed to write");

    println!("Item added!");

    let tic = Duration::from_millis(2000);

}


pub fn list() {

    clearscreen::clear().expect("failed to clear screen");

 
    pause();

}


