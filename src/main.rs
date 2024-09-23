pub mod file_operation;
use file_operation::PathToDo;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

enum Mode {
    DISPLAY,
    ADD,
    DELETE,
    EXIT,
    UNDEFINED,
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        println!("{:?}", value);
        match value {
            "1" => Mode::DISPLAY,
            "2" => Mode::ADD,
            "3" => Mode::DELETE,
            "4" => Mode::EXIT,
            _ => Mode::UNDEFINED,
        }
    }
}

fn main() -> io::Result<()> {
    let path_to_do = PathToDo {
        path: PathBuf::from("todo.txt"),
    };
    let mut input = String::new();

    loop {
        println!("1:Display, 2:Add, 3:Delete, 4:Exit");
        print!("Please type number ->");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        let mode = Mode::from(input.trim());

        match mode {
            Mode::DISPLAY => match path_to_do.try_read_lines() {
                Ok(lines) => {
                    if lines.is_empty() {
                        println!("nothing to do for now \n");
                        io::stdout().flush()?;
                    } else {
                        for todo in lines {
                            println!("{}", todo)
                        }
                    }
                    println!("\n")
                }
                Err(why) => eprintln!("Error reading file : {}", why),
            },
            Mode::ADD => {
                let mut new_todo = String::new();
                println!("please input new todo -> ");

                io::stdout().flush()?;
                io::stdin().read_line(&mut new_todo)?;

                match path_to_do.try_add_lines(&new_todo) {
                    Ok(_) => println!("success!\n"),
                    Err(why) => eprintln!("couldn't add {} : {}", new_todo, why),
                }
            }
            Mode::DELETE => {
                let to_dos = match path_to_do.try_read_lines() {
                    Ok(lines) => lines,
                    Err(e) => {
                        eprintln!("Couldn't read file!! : {}", e);
                        continue;
                    }
                };

                if to_dos.len() == 0 {
                    println!("Nothing to delete now \n");
                    io::stdout().flush()?;
                    continue;
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

                match path_to_do.find_index_opt(&delete_todo) {
                    Ok(index) => match index {
                        None => println!("not found in the list"),

                        Some(_) => match path_to_do.try_update_file(&delete_todo) {
                            Err(why) => eprintln!("failed to update the file : {}", why),
                            Ok(_) => println!("successfully delete {}", delete_todo),
                        },
                    },
                    Err(why) => eprintln!("couldn't read the file : {}", why),
                }
            }

            Mode::EXIT => {
                println!("See you!");
                break;
            }

            Mode::UNDEFINED => {
                println!("type again\n");
            }
        }
    }
    Ok(())
}
