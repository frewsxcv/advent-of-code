use Dir::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    N,
    Ne,
    Se,
    S,
    Sw,
    Nw,
}

impl Dir {
    fn from_str(s: &str) -> Dir {
        match s {
            "n" => N,
            "ne" => Ne,
            "se" => Se,
            "s" => S,
            "sw" => Sw,
            "nw" => Nw,
            _ => unreachable!(),
        }
    }

    fn is_opposite(self, other: Dir) -> bool {
        match (self, other) {
            (N, S) |
                (Ne, Sw) |
                (Se, Nw) |
                (S, N) |
                (Sw, Ne) |
                (Nw, Se) => true,
            _ => false,
        }
    }

    fn combine_thirds(self, other: Dir) -> Option<Dir> {
        match (self, other) {
            (N, Se) => Some(Ne),
            (N, Sw) => Some(Nw),
            (Ne, S) => Some(Se),
            (Ne, Nw) => Some(N),
            (Se, N) => Some(Ne),
            (Se, Sw) => Some(S),
            (S, Ne) => Some(Se),
            (S, Nw) => Some(Sw),
            (Sw, N) => Some(Nw),
            (Sw, Se) => Some(S),
            (Nw, Ne) => Some(N),
            (Nw, S) => Some(Sw),
            _ => None,
        }
    }
}

fn reduce(mut dirs: Vec<Dir>) -> Vec<Dir> {
    let mut i = 0;
    'outer: while let Some(curr_dir) = dirs.get(i).cloned() {
        let mut j = i + 1;
        while let Some(ahead_dir) = dirs.get(j).cloned() {
            if curr_dir.is_opposite(ahead_dir) {
                dirs.remove(j);
                dirs.remove(i);
                continue 'outer;
            } else if let Some(third) = curr_dir.combine_thirds(ahead_dir) {
                dirs.remove(j);
                dirs[i] = third;
                continue 'outer;
            }
            j += 1;
        }
        i += 1;
    }
    dirs
}

static INPUT: &str = include_str!("../input");

fn main() {
    let parsed = INPUT.trim().split(',').map(|n| Dir::from_str(n)).collect::<Vec<_>>();
    println!("solution: {}", reduce(parsed).len());
}

#[test]
fn test_reduce() {
    assert_eq!(vec![Ne, Ne, Ne], reduce(vec![Ne, Ne, Ne]));
    assert!(reduce(vec![Ne, Ne, Sw, Sw]).is_empty());
    assert_eq!(vec![Se, Se], reduce(vec![Ne, Ne, S, S]));
    assert_eq!(vec![S, S, Sw], reduce(vec![Se, Sw, Se, Sw, Sw]));
}
