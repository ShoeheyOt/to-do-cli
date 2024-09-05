pub mod file_operation;
use file_operation::{find_index_opt, try_add_lines, try_read_lines, try_update_file};

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
            match try_read_lines("todo.txt") {
                Ok(lines) => {
                    if lines.is_empty() {
                        println!("nothing to do for now \n");
                        io::stdout().flush()?;
                    } else {
                        for todo in lines {
                            println!("{}", todo)
                        }
                    }
                }
                Err(why) => eprintln!("Error reading file : {}", why),
            }
            println!("\n");

            continue;
        }

        if input.trim() == "2" {
            let mut new_todo = String::new();
            println!("please input new todo >> \n");
            io::stdout().flush()?;
            io::stdin().read_line(&mut new_todo)?;

            match try_add_lines(&new_todo) {
                Err(why) => eprintln!("couldn't add {} : {}", new_todo, why),
                Ok(_) => println!("success!\n"),
            }
            continue;
        }

        if input.trim() == "3" {
            if try_read_lines("todo.txt")?.len() == 0 {
                println!("Nothing to delete now \n");
                io::stdout().flush()?;
            } else {
                println!("which one? >");

                if let Ok(vector) = try_read_lines("todo.txt") {
                    for todo in vector {
                        println!("{}", todo)
                    }
                } else {
                    eprintln!("couldn't read file!!!")
                }

                let mut delete_todo = String::new();

                io::stdin().read_line(&mut delete_todo)?;

                match find_index_opt(&delete_todo) {
                    Some(_) => {
                        try_update_file(&delete_todo)?;
                        println!("successfully delete {}", delete_todo);
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
