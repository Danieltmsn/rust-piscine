fn find_average(a: f32, b: f32, c: f32) -> f32 {
    let counter = a + b + c;
    let average = counter / 3.0;
    average
}

fn main() {
    println!("{}", find_average(20.0, 32.0, 115.0))
}