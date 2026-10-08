pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut result = String::new();
            for word in name.split_whitespace() {
                if !result.is_empty() {
                    result.push(' ');
                }
                if let Some(c) = word.chars().next() {
                    result.push(c);
                    result.push('.');
                }
            }
            result
        })
        .collect()
}