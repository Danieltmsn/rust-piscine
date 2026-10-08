use std::io;

fn main() {
    println!("Whats your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to Load!");
    println!("Hello, {}!", input.trim());
}