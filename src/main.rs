use std::io;

fn main() {
    println!("Welcome to the thing (work on the title later)");
    println!("1 for translation, 2 for rotation, 3 for mirroring");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input string into an unsigned 32-bit integer
    let r: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a positive integer (1-3)");
            return;
        }
    };

    match r {
        1 => {
            println!("Translation selected.");

        }
        2 => {
            println!("Rotation selected.");

        }
        3 => {
            println!("Mirroring selected.");

        }
        _ => {
            println!("Please enter a positive integer (1-3)");
        }
    }
}
