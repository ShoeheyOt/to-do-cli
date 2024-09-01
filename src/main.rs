use std::io;

fn main()->io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() == "exit" {
        println!("Are you sure?");
    } else {
        println!("what you entered is: {}", input);
    }

    Ok(())

}
