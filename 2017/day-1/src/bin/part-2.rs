extern crate day_1;

use day_1::ascii_byte_to_val;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("solution: {}", captcha_solver(INPUT));
}

fn captcha_solver(captcha: &str) -> usize {
    let bytes = captcha.as_bytes(); // assume string is ASCII

    (0..bytes.len())
        .map(|idx| {
            (idx, (idx + (bytes.len() / 2)) % bytes.len())
        })
        .filter_map(|(idx1, idx2)| {
            if bytes[idx1] == bytes[idx2] {
                Some(ascii_byte_to_val(bytes[idx1]))
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn provided_tests() {
    assert_eq!(6, captcha_solver("1212"));
    assert_eq!(0, captcha_solver("1221"));
    assert_eq!(4, captcha_solver("123425"));
    assert_eq!(12, captcha_solver("123123"));
    assert_eq!(4, captcha_solver("12131415"));
}

#[test]
fn empty_test() {
    assert_eq!(0, captcha_solver(""));
}
