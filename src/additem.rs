use std::io::{stdin,stdout,Write};
//use serde::{Deserialize, Serialize};
//use serde_json::Result;



// Moved to model.rs
// #[derive(Serialize, Deserialize)]
// struct Todoitem {
// 	title: String,
// 	body: String,
// 	priority: String,
// 	status: String,
// 	duedate: String,
// }

use crate::models::Todoitem;


fn add(t: String, b: String, p: String, s: String, d: String) -> std::string::String {

	let newitem = Todoitem {
		title: t,
		body: b,
		priority: p,
		status: s,
		duedate: d,
	};

	//need to write nice json to our file
	
	let ni: String = serde_json::to_string(&newitem).expect("Some error when converting to json");

	//let () = ni;
	// debug
	//println!("{}", ni); // write to a file here

	//println!("{}", ni);

	ni

}

pub fn addnewitem() -> std::string::String {
	
	
	println!("Adding new item...please input the information...");
	
	let title = title();

	let body = body();

	let pri = priority(); 

	let sta = sta(); // status

	let duda = duda(); // due date

	let item = add(title,body,pri,sta,duda);

	//println!("Item Added! "); // write to a file here

	item



}


fn title() -> String {

	clearscreen::clear().expect("failed to clear screen");

	print!("Title: ");
	
	stdout().flush().unwrap();
	
	let mut t = String::new();
	
	stdin().read_line(&mut t).expect("Failed to read line");
	
	t.pop();

	t

}

fn body() -> String {

	clearscreen::clear().expect("failed to clear screen");

	print!("Body: ");
	
	stdout().flush().unwrap();
	
	let mut b = String::new();
	
	stdin().read_line(&mut b).expect("Failed to read line");
	
	b.pop();

	b

}

fn priority() -> String {

	clearscreen::clear().expect("failed to clear screen");

	println!("Priority: ");
	println!("1-Critical ");
	println!("2-High ");
	println!("3-Medium ");
	println!("4-Low ");
	println!("(press s to skip)");
	print!("Select a number: ");
	
	stdout().flush().unwrap();
	
	let mut p = String::new();
	
	stdin().read_line(&mut p).expect("Failed to read line");
	
	p.pop();

	//dbg!(&p);

	if p == "s" {

		p = "".to_string();

	} else {

		// convert to int for match statement		
		let pp: i32 = p.parse().unwrap();
		// will call our modules here


		match pp {

			1 => {
				p = "Critical".to_string()

			}
			2 => {
				p = "High".to_string()

			}
			3 => {
				p = "Medium".to_string()

			}
			4 => {			
				p = "Low".to_string()

			}
			_other => { println!("Please input a valid integer between 1-4");

			}

		}

	}

	p

}

fn sta() -> String {

	clearscreen::clear().expect("failed to clear screen");

	println!("Status: ");
	println!("1-Complete ");
	println!("2-In Progress");
	println!("3-Not Started");
	println!("4-Stalled");
	println!("(press s to skip)");
	print!("Select a number: ");
	
	stdout().flush().unwrap();
	
	let mut s = String::new();
	
	stdin().read_line(&mut s).expect("Failed to read line");
	
	s.pop();

	//dbg!(&s);

	if s == "s" {

		s = "".to_string();

	} else {

		// convert to int for match statement		
		let ss: i32 = s.parse().unwrap();
		// will call our modules here


		match ss {

			1 => {
				s = "Complete".to_string()

			}
			2 => {
				s = "In Progress".to_string()

			}
			3 => {
				s = "Not Started".to_string()

			}
			4 => {			
				s = "Stalled".to_string()

			}
			_other => { println!("Please input a valid integer between 1-4");

			}

		}

	}

	s

}

fn duda() -> String {

	clearscreen::clear().expect("failed to clear screen");

	print!("Due Date: ");
	
	stdout().flush().unwrap();
	
	let mut d = String::new();
	
	stdin().read_line(&mut d).expect("Failed to read line");
	
	d.pop();

	d

}




