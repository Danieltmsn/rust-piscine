pub fn arrange_phrase(phrase: &str) -> String {
    let count = phrase.split_whitespace().count();
    let mut words = vec![""; count];

    for word in phrase.split_whitespace() {
        let pos = word
            .chars()
            .filter_map(|c| c.to_digit(10))
            .fold(0, |acc, d| acc * 10 + d) as usize;
        words[pos - 1] = word;
    }

    let mut result = String::with_capacity(phrase.len());
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        for c in word.chars().filter(|c| !c.is_ascii_digit()) {
            result.push(c);
        }
    }
    result
}