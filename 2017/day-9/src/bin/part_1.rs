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

fn remove_garbage(input: &str) -> String {
    let mut new = String::new();
    let mut in_garbage = false;
    for c in input.chars() {
        match (in_garbage, c) {
            (false, '<') => in_garbage = true,
            (false, _) => new.push(c),
            (true, '>') => in_garbage = false,
            (true, _) => (),
        }
    }
    new
}

fn calculate_score(input: &str) -> usize {
    let input = remove_bangs(input);
    let input = remove_garbage(&input);
    let mut score = 0;
    let mut nested = 0;
    for c in input.trim().chars() {
        match c {
            '{' => {
                nested += 1;
                score += nested;
            },
            '}' => nested -= 1,
            ',' => (),
            _ => unreachable!(),
        }
    }
    score
}

fn main() {
    println!("solution: {}", calculate_score(include_str!("../input")));
}

#[test]
fn test_calculate_score() {
    assert_eq!(1, calculate_score("{}"));
    assert_eq!(6, calculate_score("{{{}}}"));
    assert_eq!(5, calculate_score("{{},{}}"));
    assert_eq!(16, calculate_score("{{{},{},{{}}}}"));
    assert_eq!(1, calculate_score("{<a>,<a>,<a>,<a>}"));
    assert_eq!(9, calculate_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(9, calculate_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(3, calculate_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
}
