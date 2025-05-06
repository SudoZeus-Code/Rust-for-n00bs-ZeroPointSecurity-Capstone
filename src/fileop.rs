use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
//use std::fs::OpenOptions;
//use std::io::{Write, Read};
use std::io::{stdin, stdout, Read, Write};

// for time delay
//use std::thread;
use std::time::Duration;

//json reading
//use serde::{Serialize,Deserialize};
//use serde_json::json::Json;
use serde_json;
use serde_json::to_writer_pretty;
use serde_json::Value;
//use std::path::Path;
//use std::error::Error;
//use std::fs::File
use std::io::{BufReader,BufWriter};
//use std::path::Path;

// for list
use std::fs;


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

    // //https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
    // let mut file = File::open("library.json").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();

    // // Using Value here instead of Todoitem??
    // let json: Todoitem = serde_json::from_str(&data).unwrap();
    
    // //https://whoisryosuke.com/blog/2022/parsing-json-with-rust
    // println!("Title:{}\nBody:{}", json["title"], json["body"][0]);
    
    let file_path = "library.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldnt find or load file.");
    untyped_example(&contents);
    pause();

}

fn untyped_example(json_data: &str) -> Result<(), Box<dyn std::error::Error>> {

    //let v: Value = serde_json::from_str(json_data)?;

    //https://www.reddit.com/r/rust/comments/cge2gd/array_of_objects_in_json_serde/
    // Read into a vector using the struct Todoitem
    //let v: Vec<Todoitem> = serde_json::from_str(json_data)?;
    


  
    use std::collections::HashMap;

    //https://williamhuey.github.io/posts/rust-serde-iterating-over-json-keys/
    let v: Vec<HashMap<String, Value>> = serde_json::from_str(json_data)?;

    dbg!(&v);
    //let () = v; // Vec<Todoitem>

    // List whole TodoItem
    //println!("{:#?}", v[1]);

    
    
    //https://users.rust-lang.org/t/access-an-element-of-a-vector/45320/3
    //println!("Title:{:#?}", v[0].title);
    //println!("Body:{:#?}", v[0].title);
    
    //https://www.reddit.com/r/learnrust/comments/c0jv1t/comment/er5dng5/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    // for (key, value) in v {
    //     println!{"{:?}:{:?}", key, value};

    // }
    
    for item in v.iter() {
        println!("{}", &item["title"]);
        dbg!(item.keys());
    }

    Ok(())
}


