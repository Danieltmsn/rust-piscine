use std::io;

fn main() {
    let mut counter = 0;

    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        counter += 1;
        if input.trim() == "The letter e" {
            break;
        }

    
    }
    println!("Number of trials: {}", counter)
    
}
