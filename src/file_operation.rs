use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// this is a function to read lines from text file
///
///
///
///
///
pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    // let mut vector = Vec::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    // for line in reader.lines() {
    //     vector.push(line?);
    // }

    Ok(reader.lines().filter_map(|line| line.ok()).collect())
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
    match read_lines("todo.txt") {
        Err(why) => {
            println!("couldn't read the lines : {}", why);
            None
        }
        Ok(vector) => vector.iter().position(|todo| todo == delete_todo.trim()),
    }
}

pub fn update_file(delete_todo: &String) -> io::Result<()> {
    let path = Path::new("todo.txt");
    let todo_list: Vec<String> = match read_lines("todo.txt") {
        Err(why) => {
            println!("couldn't read lines: {}", why);
            vec![]
        }
        Ok(vector) => vector
            .into_iter()
            .filter(|todo| *todo != delete_todo.trim())
            .collect(),
    };
    // .into_iter()
    // .filter(|todo| *todo != delete_todo.trim())
    // .collect();

    let mut file_truncate = match OpenOptions::new().write(true).truncate(true).open(path) {
        Err(why) => panic!("couldn't open the file {} : {}", path.display(), why),
        Ok(file) => file,
    };

    for todo in todo_list {
        writeln!(file_truncate, "{}", todo)?;
    }
    Ok(())
}
