use std::fs::OpenOptions;
use std::path::Path;
use std::io;
use std::io::prelude::*;

fn main()->io::Result<()> {
    let path = Path::new("todo.txt");

    let display = path.display();

    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("couldn't find {} : {}", display, why),
        Ok(file)=> file,
    };

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    if input.trim() == "exit" {
        println!("Are you sure?");

        io::stdin().read_line(&mut input)?;

        println!("See you soon: {}", input)
    } else {

        match file.write(input.as_bytes()){
            Err(why) => panic!("couldn't add {} to {}: {}", input.trim(),display,why),
            Ok(_) => print!("successfully wrote {} to {}", input.trim(), display) 
        }
    }
    Ok(())
}
