use std::io::{self, Write};

fn main() {
    let sea_creatures = ["Marlin", "Dory", "Gill", "Nemo", "Bubbles", "Bloat"];

    print!("Enter the name of the sea creature you are looking for: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let mut found = false;
    for creature in sea_creatures.iter() {
        if creature == input {
            println!("Found {}!", creature);
            found = true;
            break;
        }
    }

    if !found {
        println!("{} not found.", input);
    }
}
