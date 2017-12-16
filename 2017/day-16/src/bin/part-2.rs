#![feature(conservative_impl_trait)]
#![feature(slice_rotate)]

use std::collections::HashMap;

const NUM_PROGRAMS: usize = 16;

static PROGRAMS_INIT: [u8; NUM_PROGRAMS] = [
    b'a', b'b', b'c', b'd',
    b'e', b'f', b'g', b'h',
    b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p',
];

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Move {
    #[inline]
    fn apply(self, programs: &mut [u8]) {
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

    fn from_str(input: &str) -> Self {
        let mut chars = input.chars();
        match chars.next().unwrap() {
            's' => Move::Spin(chars.as_str().parse().unwrap()),
            'x' => {
                let mut nums = chars.as_str().split('/');
                Move::Exchange(
                    nums.next().unwrap().parse().unwrap(),
                    nums.next().unwrap().parse().unwrap(),
                )
            },
            'p' => {
                let a = chars.next().unwrap();
                let _ = chars.next();
                let b = chars.next().unwrap();
                Move::Partner(a as u8, b as u8)
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut programs = PROGRAMS_INIT.clone();

    let input = include_str!("../input").trim();

    let moves = reduce(parse(input));

    let (first_instance, cycle_len) = find_cycle(&mut programs, &moves);

    let n = (1_000_000_000 - first_instance) % cycle_len;

    let mut programs = PROGRAMS_INIT.clone();
    for _ in 0..n {
        run_dance(&mut programs, &moves);
    }

    print(&programs);
}

fn find_cycle(programs: &mut [u8], moves: &[Move]) -> (usize, usize) {
    let mut map = HashMap::new();
    for i in 0..10000000 {
        if let Some(prev_i) = map.get(programs).cloned() {
            return (prev_i, i - prev_i);
        } else {
            map.insert(programs.to_owned(), i);
            run_dance(programs, moves);
        }
    }
    unreachable!()
}

fn run_dance(programs: &mut [u8], moves: &[Move]) {
    for move_ in moves {
        move_.apply(programs);
    }
}

fn print(programs: &[u8]) {
    for program in programs.iter() {
        print!("{}", *program as char);
    }
    println!();
}

fn parse(input: &str) -> Vec<Move> {
    input.split(',').map(Move::from_str).collect::<Vec<_>>()
}

// reduce all the spins into one spin
// this turned out to be not a good use of my time, but oh well
fn reduce(moves: Vec<Move>) -> Vec<Move> {
    let mut curr_spin = 0;
    let mut new_moves = vec![];
    for move_ in moves {
        match move_ {
            Move::Spin(s) => {
                curr_spin = (curr_spin + s) % NUM_PROGRAMS;
            },
            Move::Exchange(a, b) => {
                new_moves.push(
                    Move::Exchange(
                        (a + NUM_PROGRAMS - curr_spin) % NUM_PROGRAMS,
                        (b + NUM_PROGRAMS - curr_spin) % NUM_PROGRAMS,
                    )
                );
            },
            m => new_moves.push(m),
        }
    }
    new_moves.push(Move::Spin(curr_spin));
    new_moves
}
