pub mod file_operation;
use file_operation::{find_index_opt, try_add_lines, try_read_lines, try_update_file};

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut input = String::new();

    loop {
        println!("1:Display, 2:Add, 3:Delete, 4:Exit");
        print!("Please type number ->");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => match try_read_lines("todo.txt") {
                Ok(lines) => {
                    if lines.is_empty() {
                        println!("nothing to do for now\n");
                        io::stdout().flush()?;
                    } else {
                        for todo in lines {
                            print!("{}", todo)
                        }
                        println!("\n")
                    }
                }
                Err(why) => eprintln!("Error reading file : {}", why),
            },
            "2" => {
                let mut new_todo = String::new();
                println!("please input new todo -> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut new_todo)?;

                match try_add_lines(&new_todo) {
                    Err(why) => eprintln!("couldn't add {} : {}", new_todo, why),
                    Ok(_) => println!("success!\n"),
                }
            }
            "3" => {
                let to_dos = match try_read_lines("todo.txt") {
                    Ok(lines) => lines,
                    Err(e) => {
                        eprintln!("Couldn't read file!! : {}", e);
                        continue;
                    }
                };

                if to_dos.len() == 0 {
                    println!("Nothing to delete now \n");
                    io::stdout().flush()?;
                } else {
                    println!("You have below remained tasks");
                    for todo in to_dos {
                        println!("{}", todo);
                    }
                    print!("\n");
                    print!("which one do you want to delete? ->");
                    io::stdout().flush()?;
                };

                let mut delete_todo = String::new();

                io::stdin().read_line(&mut delete_todo)?;

                match find_index_opt(&delete_todo) {
                    Ok(index) => match index {
                        None => println!("not found in the list"),

                        Some(_) => match try_update_file(&delete_todo) {
                            Err(why) => eprintln!("failed to update the file : {}", why),
                            Ok(_) => println!("successfully delete {}", delete_todo),
                        },
                    },
                    Err(why) => eprintln!("couldn't read the file : {}", why),
                }
            }
            "4" => {
                println!("See you!");
                break;
            }
            _ => {
                println!("type again\n");
            }
        }
    }
    Ok(())
}
