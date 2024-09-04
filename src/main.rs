pub mod file_operation;
use file_operation::{find_position, read_lines, update_file};

use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("todo.txt");

    let display = path.display();

    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("couldn't find {} : {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();

    loop {
        println!("1:Display, 2:Add, 3:Delete, 4:Exit");
        println!("Please type number ->");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "1" {
            if read_lines("todo.txt").len() == 0 {
                println!("nothing to do for now \n");
                io::stdout().flush()?;
            } else {
                for todo in read_lines("todo.txt") {
                    println!("{}", todo)
                }
                println!("\n");
            }
            continue;
        }

        if input.trim() == "2" {
            let mut new_todo = String::new();
            println!("please input new todo >> \n");
            io::stdout().flush()?;
            io::stdin().read_line(&mut new_todo)?;

            match file.write(new_todo.as_bytes()) {
                Err(why) => panic!("couldn't add {} to {}: {}", new_todo.trim(), display, why),
                Ok(_) => println!("successfully wrote {} to {} \n", new_todo.trim(), display),
            }
            continue;
        }

        if input.trim() == "3" {
            if read_lines("todo.txt").len() == 0 {
                println!("Nothing to delete now \n");
                io::stdout().flush()?;
            } else {
                println!("which one? >");

                for todo in read_lines("todo.txt") {
                    println!("{}", todo)
                }

                let mut delete_todo = String::new();

                io::stdin().read_line(&mut delete_todo)?;

                match find_position(&delete_todo) {
                    Some(index) => {
                        match update_file(&delete_todo, &display, path) {
                            Err(why) => {
                                println!("couldn't delete from file : {} index is {}", why, index)
                            }
                            Ok(_) => println!("successfully delete {}", delete_todo),
                        };
                    }

                    None => println!("not found\n"),
                }
            }

            continue;
        }

        if input.trim() == "4" {
            println!("See you!");
            break;
        }
    }
    Ok(())
}
