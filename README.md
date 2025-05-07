# Rust for n00bs (ZeroPoint Security Course) Capstone Project 

## to-do list application

My code base consist of 4 files.

### main.rs
Main looping console for the program. Calls functions in additem.rs and fileop.rs. Simple and clean. 

### models.rs
Contains the struct for our JSON objects. 

### additem.rs
Additem is used for adding TO-DO items to the list. Contains functions for retrieving json feilds from the user and formating them correctly. Also has a function that calls a crate that contains the struct for the json object and adds all user input to the json object. Returns a json string to main, that is then passed to the function write() in fileops.rs which writes it to a file. 

### fileop.rs
Fileop is used to perform all file operations. This includes exists() which is ran at program start to verify the json file exists and if not creates one. Write() writes our json object to the file. List() lists all to-do json objects in the file. Show() takes a title from user input and retrieves the specific to-do item from the file. Delete() removes a to-do item from the file. 

```
The capstone project is to create a to-do list application using all the knowledge and skills presented throughout this course.

A to-do item should contain at least a:

    Short title/headline.
    Detailed body/description.


The application should prompt the user to:

    Add a new to-do item.
    List the title/headline of every to-do item.
    Display the body of a specific to-do item.
    Delete a to-do item.


You may optionally add any additional features you like, such as priority (e.g. urgent), status (e.g. not started/started/complete), and due date.  To get the maximum value from the final project, it will ideally have module, console, and unit tests components.
```
