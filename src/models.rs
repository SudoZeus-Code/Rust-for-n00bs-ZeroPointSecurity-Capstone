use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todoitem {
    pub title: String,
    pub body: String,
    pub priority: String,
    pub status: String,
    pub duedate: String,
}