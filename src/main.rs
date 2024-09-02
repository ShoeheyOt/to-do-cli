use std::{fs::File, io::{self, Write}, path::Path};

fn main()->io::Result<()> {
    let path = Path::new("todo.txt");
    let display = path.display();

    let mut file = match File::create(&path){
        Err(why) => panic!("couldn't find {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() == "exit" {
        println!("Are you sure?");
        io::stdin().read_line(&mut input)?;
        println!("See you soon: {}", input)
    } else {
        match file.write(input.trim().as_bytes()){
            Err(why) => panic!("couldn't add {} to {}: {}", input.trim(),display,why),
            Ok(_) => print!("successfully wrote {} to {}", input.trim(), display)
        }
        
    }

    Ok(())

}
