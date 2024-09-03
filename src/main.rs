use std::fs::{read_to_string, OpenOptions};
use std::path::Path;
use std::io;
use std::io::prelude::*;

fn read_lines(filename: &str)->Vec<String> {
    let mut vector = Vec::<String>::new();
    for line in read_to_string(filename).unwrap().lines(){
        vector.push(line.to_string());
    }
    
    vector
}

fn main()->io::Result<()> {
    let path = Path::new("todo.txt");

    let display = path.display();

    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("couldn't find {} : {}", display, why),
        Ok(file)=> file,
    };

    let mut input = String::new();
    println!("1:Display, 2:Add, 3:Delete, 4:Exit");
    println!("Please type number ->");
    io::stdin().read_line(&mut input)?;

    if input.trim() == "4" {
        println!("Are you sure?");

        io::stdin().read_line(&mut input)?;

        println!("See you soon")
    } else if input.trim() == "2" {
        let mut new_todo = String::new();
        io::stdin().read_line(&mut new_todo)?;
        let line_number = if read_lines("todo.txt").len() == 0 {
            1
        } else {
            read_lines("todo.txt").len() + 1
        };
        let add_todo = line_number.to_string() + " " + &new_todo;
        match file.write(add_todo.as_bytes()){
            Err(why) => panic!("couldn't add {} to {}: {}", new_todo.trim(),display,why),
            Ok(_) => println!("successfully wrote {} to {}", new_todo.trim(), display) 
        }
    } else if input.trim() == "1" {
        for todo in read_lines("todo.txt") {
            println!("{}", todo)
        }
    }
    Ok(())
}
