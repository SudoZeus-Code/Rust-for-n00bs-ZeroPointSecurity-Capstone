
//use std::io::{stdin,stdout,write};
use std::io::{stdin,stdout,Write};
//use std::io::Write;

// file checks
mod fileop;
use fileop::exists;
use fileop::write;
use fileop::list;

mod additem;
use additem::addnewitem;
mod models;

//mod listitem;

//mod showitem;

//mod delitem;

fn main() {

	
	exists();
	
	loop {

		clearscreen::clear().expect("failed to clear screen");

		

		println!("To-Do List:\n");

		//Create menu of options here
		println!("1️⃣  Add new item");
	
		println!("2️⃣  List all items");
	
		println!("3️⃣  Show specific item");
	
		println!("4️⃣  Delete an item");
	
		println!("5️⃣  Exit");
		
		// get user input on same line 
		print!("Choice: ");
		stdout().flush().unwrap();
		let mut c = String::new();
		stdin().read_line(&mut c).expect("Failed to read line");	
		c.pop(); // get rid of trailing new line
	
		// check to ensure input is numeric
		let cc = &c.parse::<i32>();
			match cc {
				Ok(_) => println!("\n"), // this is hella sloppy
				Err(e) => println!("ERROR:`{}`\n Please input a valid integer between 1-4",e),
			}
	
		// convert to int for match statement		
		let cm: i32 = c.parse().unwrap();
		// will call our modules here
		match cm {
			
			1 => {
	
				let item = addnewitem();

										
				write(item.to_string());
	
			}
			2 => {
				
				list()
				
			}
			3 => {
				println!("choice {} selected", c);
			}
			4 => {			
				println!("choice {} selected", c);
			}
			5 => {
				println!("EXITING!");
				std::process::exit(1)
			}
			_other => { println!("Please input a valid integer between 1-4");
	
			}
	
	
		}


	}

}

