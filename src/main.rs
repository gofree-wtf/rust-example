use std::io;

fn main() {
    println!("Hello, world!");
    println!("Input string:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).
        expect("Failed to read line");

    println!("Got it: {}", input);
}
