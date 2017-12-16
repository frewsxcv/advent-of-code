#![feature(conservative_impl_trait)]
#![feature(slice_rotate)]

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn apply(self, programs: &mut [char]) {
        match self {
            Move::Spin(a) => {
                let rotation_amount = programs.len() - a;
                programs.rotate(rotation_amount);
            }
            Move::Exchange(a, b) => programs.swap(a, b),
            Move::Partner(a, b) => {
                let a_idx = programs.iter().position(|p| *p == a).unwrap();
                let b_idx = programs.iter().position(|p| *p == b).unwrap();
                programs.swap(a_idx, b_idx);
            },
        }
    }
}

fn main() {
    let mut programs = [
        'a', 'b', 'c', 'd',
        'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p',
    ];

    let input = include_str!("../input").trim();

    for command in parse(input) {
        command.apply(&mut programs);
    }

    println!("solution: {}", programs.into_iter().collect::<String>());
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item=Move> + 'a {
    input.split(',').map(|s| parse_move(s))
}

fn parse_move(input: &str) -> Move {
    let mut chars = input.chars();
    match chars.next().unwrap() {
        's' => {
            let mut a = String::new();
            a.push(chars.next().unwrap());
            if let Some(next) = chars.next() {
                a.push(next);
            }
            Move::Spin(a.parse().unwrap())
        },
        'x' => {
            let mut a = String::new();
            a.push(chars.next().unwrap());
            let next = chars.next().unwrap();
            if next != '/' {
                a.push(next);
                let _ = chars.next().unwrap();
            }
            let mut b = String::new();
            b.push(chars.next().unwrap());
            if let Some(next) = chars.next() {
                b.push(next);
            }
            Move::Exchange(a.parse().unwrap(), b.parse().unwrap())
        },
        'p' => {
            let a = chars.next().unwrap();
            let _ = chars.next();
            let b = chars.next().unwrap();
            Move::Partner(a, b)
        },
        _ => unreachable!(),
    }
}
