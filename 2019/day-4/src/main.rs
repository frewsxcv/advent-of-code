const INPUT_LOW: u64 = 245182;
const INPUT_HIGH: u64 = 790572;

fn has_adjacent_digits(n: u64) -> bool {
    n.to_string().as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn does_not_decrease(n: u64) -> bool {
    n.to_string().as_bytes().windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    let mut count = 0;

    for n in INPUT_LOW..INPUT_HIGH {
        if has_adjacent_digits(n) && does_not_decrease(n) {
            count += 1;
        }
    }

    println!("part 1: {}", count)
}

#[cfg(test)]
mod test {
    use super::does_not_decrease;

    #[test]
    fn test_does_not_decrease() {
        assert!(does_not_decrease(123456));
        assert!(does_not_decrease(666666));
        assert!(!does_not_decrease(666566));
    }
}
