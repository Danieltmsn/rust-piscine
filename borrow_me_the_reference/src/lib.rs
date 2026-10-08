pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut to_delete = 0;

    for c in s.chars() {
        if c == '-' {
            result.pop();
        } else if c == '+' {
            to_delete += 1;
        } else if to_delete > 0 {
            to_delete -= 1;
        } else {
            result.push(c);
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        let result = if let Some((a, b)) = s.split_once('+') {
            a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap()
        } else if let Some((a, b)) = s.split_once('-') {
            a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()
        } else {
            continue;
        };
        *s = result.to_string();
    }
}