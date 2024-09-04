pub mod file_operation;
use file_operation::{add_lines, find_position, read_lines, update_file};

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
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

            add_lines(new_todo);
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
                        match update_file(&delete_todo) {
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
