# Todo CLI Tool

A simple command-line interface for managing to-do list stored in a text file.

## Features

- Display all to-do items
- Add new to-do items
- Delete existing to-do items

## Usage

1. Run the program using `cargo run` or build with `cargo build`
2. interact with the CLI menu

```CLI

1: Display
2: Add
3: Delete
4: Exit

```

3. Enter the corresponding number to perform action.

## Functionality Breakdown

### Display

- Reads all lines from "todo.txt"
- Prints each line if the file is not empty

### Add

- Prompts for a new to-do item
- Adds the item to the end of "todo.txt"

### Delete

- Displays existing items
- Allows selection of an item to delete
- Updates "todo.txt" by removing the selected item

### Error Handling

- Uses Rust's Result type for error handling
- Provides informative error messages for various scenarios

## Building and Running

To build and run the program:

1. Clone this repository
2. Open a terminal in the project directory
3. Run cargo run

## Contributing

Contributors are welcome! Please feel free to submit pull request or report issues.

## License

This project is licensed under the MIT License
