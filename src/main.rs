use std::io;

fn main()->io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("what you entered is: {}", input);

    Ok(())

}
