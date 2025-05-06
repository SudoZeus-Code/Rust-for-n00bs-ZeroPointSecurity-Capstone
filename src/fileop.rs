use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
use std::io::{stdin, stdout, Read, Write};
use std::time::Duration;
use serde_json;
use serde_json::to_writer_pretty;
use serde_json::Value;
use std::io::{BufReader,BufWriter};
use std::fs;
use std::collections::HashMap;

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

    let _tic = Duration::from_millis(2000);

}


pub fn list() {

    clearscreen::clear().expect("failed to clear screen");

    let file_path = "library.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldnt find or load file.");


    let v: Vec<HashMap<String, Value>> = serde_json::from_str(&contents).expect("Failed to read to HashMap");

    //https://williamhuey.github.io/posts/rust-serde-iterating-over-json-keys/
    for item in v.iter() {
        println!("Title:{}", &item["title"]);
        println!("Body:{}", &item["body"]);
        println!("Priority:{}", &item["priority"]);
        println!("Status:{}", &item["status"]);
        println!("Due Date:{}", &item["duedate"]);
        println!("\n");
        //dbg!(item.keys());
    }

    pause();

}


