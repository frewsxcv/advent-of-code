const INPUT_LOW: u64 = 245182;
const INPUT_HIGH: u64 = 790572;

mod part1 {
    pub fn has_adjacent_digits(n: u64) -> bool {
        n.to_string().as_bytes().windows(2).any(|w| w[0] == w[1])
    }

    pub fn does_not_decrease(n: u64) -> bool {
        n.to_string().as_bytes().windows(2).all(|w| w[0] <= w[1])
    }
}

mod part2 {
    pub fn has_adjacent_digits(n: u64) -> bool {
        let mut s = n.to_string();
        s.insert(0, '*');
        s.push('*');
        s.as_bytes().windows(4).any(|w| {
            w[0] != w[1] && w[1] == w[2] && w[2] != w[3]
        })
    }

    pub fn does_not_decrease(n: u64) -> bool {
        n.to_string().as_bytes().windows(2).all(|w| w[0] <= w[1])
    }
}

fn main() {
    let mut count = 0;
    for n in INPUT_LOW..INPUT_HIGH {
        if part1::has_adjacent_digits(n) && part1::does_not_decrease(n) {
            count += 1;
        }
    }
    println!("part 1: {}", count);

    count = 0;
    for n in INPUT_LOW..INPUT_HIGH {
        if part2::has_adjacent_digits(n) && part2::does_not_decrease(n) {
            count += 1;
        }
    }
    println!("part 2: {}", count);
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_part1_does_not_decrease() {
        assert!(part1::does_not_decrease(123456));
        assert!(part1::does_not_decrease(666666));
        assert!(!part1::does_not_decrease(666566));
    }
}
