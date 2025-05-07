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


    //let v: Vec<HashMap<String, Value>> = serde_json::from_str(&contents).expect("Failed to read to HashMap");
    // Utilizing Vec<Value> instead of a hashmmap
    let v: Vec<Value> = serde_json::from_str(&contents).expect("Failed to read to vector");

    //dbg!(&v);

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


pub fn show() {

    clearscreen::clear().expect("failed to clear screen");

    let file_path = "library.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldnt find or load file.");

    // change this to Vec<Value>
    //let v: Vec<HashMap<String, Value>> = serde_json::from_str(&contents).expect("Failed to read to HashMap");
    let v: Vec<Value> = serde_json::from_str(&contents).expect("Failed to read to HashMap");

    print!("Enter a title to search for: ");
    stdout().flush().unwrap();
    let mut c = String::new();
    stdin().read_line(&mut c).expect("Failed to read line");	
    c.pop(); // get rid of trailing new line

    
    for item in v.iter() {

        if *c == item["title"] {
            println!("Title:{}", &item["title"]);
            println!("Body:{}", &item["body"]);
            println!("Priority:{}", &item["priority"]);
            println!("Status:{}", &item["status"]);
            println!("Due Date:{}", &item["duedate"]);
            println!("\n");
        }
    }

    pause();



}


// pub fn delete() {

//     clearscreen::clear().expect("failed to clear screen");

//     let file_path = "library.json".to_owned();

//     let contents = fs::read_to_string(file_path).expect("Couldnt find or load file.");

//     //let mut v: Vec<Value> = serde_json::from_str(&contents).expect("Failed to read to HashMap");
//     let mut json_resp: serde_json::Value = serde_json::from_str(&contents).expect("Json parsing error");

//     //https://crates.io/crates/json_value_remove

//     extern crate json_value_remove;

//     use json_value_remove::Remove;
//     use serde_json::Value;

//     //Value::String("value1.2".to_String())

//     dbg!(&json_resp);
//     json_resp.remove("lightning").unwrap();
//     dbg!(&json_resp);
//     pause();

//     // maybe try the array? instead of the object example
// }
 


pub fn delete() {

    clearscreen::clear().expect("failed to clear screen");

    let file_path = "library.json".to_owned();

    let contents = fs::read_to_string(&file_path).expect("Couldnt find or load file.");

    

    let mut json_array: Vec<Value> = serde_json::from_str(&contents).expect("Failed to read to HashMap");


    print!("(!) Enter a title to delete: ");
    stdout().flush().unwrap();
    let mut c = String::new();
    stdin().read_line(&mut c).expect("Failed to read line");	
    //c.pop(); // get rid of trailing new line
    let title_be_gone = c.trim();

    let original_len = json_array.len();

    json_array.retain(|item| {
        item.get("title")
            .and_then(|t| t.as_str())
            .map(|t| t != title_be_gone)
            .unwrap_or(true)
    });


    if json_array.len() == original_len {
        println!("no item found with the title '{}'", title_be_gone);
    } else {
        println!("Item with title '{}' has been removed.", title_be_gone);


        // write new to library,json
        let new_json = serde_json::to_string_pretty(&json_array).expect("Failed to serialize JSON");
        fs::write(file_path, new_json).expect("Failed to write JSON file.");

    }

    pause();


} 
