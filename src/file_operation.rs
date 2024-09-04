use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// this is a function to read lines from text file
///
///
///
///
///
pub fn read_lines(filename: &str) -> Vec<String> {
    let mut vector = Vec::<String>::new();
    for line in read_to_string(filename).unwrap().lines() {
        vector.push(line.to_string());
    }
    vector
}
/// Adds a new todo item to the end of the "todo.txt" file.
///
/// ## Arguments
///
/// * `new_todo`: The string representing the new todo item to add.
///
/// ## Returns
///
/// An `io::Result<()>` indicating whether the operation was successful.
/// Returns `Ok(())` if the operation succeeds, or an `Err` containing the I/O error if it fails.
///
///
/// ## Examples
/// add _lines("Buy milk".to_string());
/// ```
pub fn add_lines(new_todo: &str) -> io::Result<()> {
    let path = Path::new("todo.txt");

    let mut file = OpenOptions::new().append(true).open(path)?;

    file.write(new_todo.as_bytes())?;

    Ok(())
}

pub fn find_position(delete_todo: &String) -> Option<usize> {
    read_lines("todo.txt")
        .iter()
        .position(|todo| todo == delete_todo.trim())
}

pub fn update_file(delete_todo: &String) -> io::Result<()> {
    let path = Path::new("todo.txt");
    let todo_list: Vec<String> = read_lines("todo.txt")
        .into_iter()
        .filter(|todo| *todo != delete_todo.trim())
        .collect();

    let mut file_truncate = match OpenOptions::new().write(true).truncate(true).open(path) {
        Err(why) => panic!("couldn't open the file {} : {}", path.display(), why),
        Ok(file) => file,
    };

    for todo in todo_list {
        writeln!(file_truncate, "{}", todo)?;
    }
    Ok(())
}
