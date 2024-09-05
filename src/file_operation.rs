use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Reads all lines from a file and returns them as a vector of strings.
///
/// ## Arguments
///
/// * `filename`: The path to the file to read. Can be any type that implements `AsRef<Path>`.
///
/// ## Returns
///
/// A `Result` containing a `Vec<String>` if successful, or an `io::Error` if an error occurs.
///
/// ## Errors
///
/// This function will return an `io::Error` if:
/// - The file cannot be opened (e.g., due to permissions issues)
/// - There's an I/O error while reading the file
/// - Any line in the file contains invalid UTF-8 data
///
/// ## Examples
///
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// let filename = Path::new("example.txt");
/// match read_lines(filename) {
///   Ok(lines) => println!("Lines: {:?}", lines),
///   Err(e) => eprintln!("Error reading file: {}", e),
/// }
///
pub fn try_read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

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
///
/// ``````
///  add _lines("Buy milk".to_string());
pub fn try_add_lines(new_todo: &str) -> io::Result<()> {
    let path = Path::new("todo.txt");

    let mut file = OpenOptions::new().append(true).open(path)?;

    file.write(new_todo.as_bytes())?;

    Ok(())
}

///Find the index number from text file
///
/// ## Arguments
/// * `searched_todo` : The string representing the item to find its index
///
/// ## Returns
///
/// An Option<usize> representing the position of the found todo item
/// - `Some(index)` if the todo item is found, where `index` is from zero.
/// - `None` if the todo item is not found or if there is an error reading the file
///
/// ## Example
/// ```
///  let todo_to_find = String::from("Buy milk");
///  match find_position(&todo_to_find) {
///     Some(index) => println!("Todo found at position: {}", position),
///     None => println!("Todo not found or error reading file"),
///  }
/// ```
pub fn find_index_opt(searched_todo: &String) -> Option<usize> {
    match try_read_lines("todo.txt") {
        Err(why) => {
            eprintln!("couldn't read the lines : {}", why);
            None
        }
        Ok(vector) => vector.iter().position(|todo| todo == searched_todo.trim()),
    }
}

/// OverWrite the todo list(exclude argument: `delete_todo`) to the file
///
/// ## Argument
///  * `delete_todo` A reference representing the todo item which is desired to be deleted
///
/// ## Returns
///
/// An `io::Result<()>` indicating success or failure:
/// - `Ok(())` if the operation succeeds
/// - `Err(io::Error)` if there's an error reading or writing the file
///
/// ## Errors
///
/// This function may return an `io::Error` due to various reasons:
/// - Failure to read the existing todo list file
/// - Unable to open the file for writing
/// - Error during truncation of the file
/// - Failure to write the updated list back to the file
///
/// ## Examples
/// ```
///  use std::path::Path;
///  let delete_todo = String::from("Buy milk");
///  match try_update_file(&delete_todo) {
///      Ok(_) => println!("Todo list updated successfully"),
///      Err(e) => eprintln!("Error updating todo list: {}", e),
///  }
///
pub fn try_update_file(delete_todo: &String) -> io::Result<()> {
    let path = Path::new("todo.txt");

    let todo_list: io::Result<Vec<String>> = match try_read_lines("todo.txt") {
        Err(why) => {
            eprintln!("couldn't read lines: {}", why);
            Err(why)
        }
        Ok(vector) => Ok(vector
            .into_iter()
            .filter(|todo| *todo != delete_todo.trim())
            .collect::<Vec<String>>()),
    };

    let mut file_truncate = OpenOptions::new().write(true).truncate(true).open(path)?;

    for todo in todo_list? {
        writeln!(file_truncate, "{}", todo)?;
    }
    Ok(())
}
