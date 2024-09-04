use std::fs::{read_to_string, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::{Display, Path};

fn read_lines(filename: &str) -> Vec<String> {
    let mut vector = Vec::<String>::new();
    for line in read_to_string(filename).unwrap().lines() {
        vector.push(line.to_string());
    }

    vector
}

fn find_position(delete_todo: &String) -> Option<usize> {
    read_lines("todo.txt")
        .iter()
        .position(|todo| todo == delete_todo.trim())
}

fn update_file(delete_todo: &String, display: &Display, path: &Path) -> io::Result<()> {
    let todo_list: Vec<String> = read_lines("todo.txt")
        .into_iter()
        .filter(|todo| *todo != delete_todo.trim())
        .collect();

    let mut file_truncate = match OpenOptions::new().write(true).truncate(true).open(path) {
        Err(why) => panic!("couldn't open the file {} : {}", display, why),
        Ok(file) => file,
    };

    for todo in todo_list {
        writeln!(file_truncate, "{}", todo)?;
    }
    Ok(())
}

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
