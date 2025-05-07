use std::io::{stdin,stdout,Write};

// file operations
mod fileop;
use fileop::{exists, write, list, show, delete};

// add item 
mod additem;
use additem::addnewitem;
mod models;


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
	
				let item = addnewitem(); // could clean this up 
				
				write(item.to_string()); // could clean this up
	
			}
			2 => {
				
				list()
				
			}
			3 => {

				show()

			}
			4 => {			

				delete()

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

