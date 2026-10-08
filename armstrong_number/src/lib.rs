pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let digits = nb.to_string();
    let power = digits.len() as u32;

    let sum: u64 = digits
        .chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(power))
        .sum();

    if sum == nb as u64 {
        Some(nb)
    } else {
        None
    }
}