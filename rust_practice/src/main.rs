use std::io;

fn main() {
    println!("First name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to Load!");
    
    println!("Last name?");

    let mut surname = String::new();
    io::stdin().read_line(&mut surname).expect("Failed to Load!");

    println!("Full name: {} {}", name.trim(), surname.trim());





}