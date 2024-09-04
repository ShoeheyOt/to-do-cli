use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};
use std::path::{Display, Path};

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut vector = Vec::<String>::new();
    for line in read_to_string(filename).unwrap().lines() {
        vector.push(line.to_string());
    }

    vector
}

pub fn find_position(delete_todo: &String) -> Option<usize> {
    read_lines("todo.txt")
        .iter()
        .position(|todo| todo == delete_todo.trim())
}

pub fn update_file(delete_todo: &String, display: &Display, path: &Path) -> io::Result<()> {
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
