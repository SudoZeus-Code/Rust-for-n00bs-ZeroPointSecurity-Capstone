
use serde::{Deserialize, Serialize};

struct TodoItem {
	title: String,
	body: String,
	priority: String,
	status: String,
	duedate: String,

}

pub fn additem(t: String, b: String: p: String, s: String, d: String) {

	let newitem = ToDoItem {
		title: t,
		body: b,
		priority: p,
		status: s,
		duedate: d,
	}
	
	let newitem = serde_json::to_string(&newitem)?;



	println!("{}", newitem);


}
