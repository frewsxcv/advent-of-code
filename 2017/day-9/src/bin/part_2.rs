fn remove_bangs(input: &str) -> String {
    let mut new = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '!' {
            let _ = chars.next();
        } else {
            new.push(c);
        }
    }
    new
}

fn garbage_count(input: &str) -> usize {
    let mut in_garbage = false;
    let mut count = 0;
    for c in input.chars() {
        match (in_garbage, c) {
            (false, '<') => in_garbage = true,
            (false, _) => (),
            (true, '>') => in_garbage = false,
            (true, _) => count += 1,
        }
    }
    count
}

fn calculate_score(input: &str) -> usize {
    let input = remove_bangs(input);
    garbage_count(&input)
}

fn main() {
    println!("solution: {}", calculate_score(include_str!("../input")));
}
