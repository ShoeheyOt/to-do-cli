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
        let mut todo = String::new();
        io::stdin().read_line(&mut todo)?;

        match file.write(todo.as_bytes()){
            Err(why) => panic!("couldn't add {} to {}: {}", input.trim(),display,why),
            Ok(_) => print!("successfully wrote {} to {}", input.trim(), display) 
        }
    } else if input.trim() == "1" {
        println!("display: {:?}", read_lines("todo.txt"))

    }
    Ok(())
}
