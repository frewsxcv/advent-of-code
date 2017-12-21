use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl Particle {
    fn from_line(line: &str) -> Self {
        let mut iter = line.trim().split(", ");
        let mut p_iter = iter.next().unwrap().trim_left_matches("p=<").trim_right_matches(">").split(",");
        let mut v_iter = iter.next().unwrap().trim_left_matches("v=<").trim_right_matches(">").split(",");
        let mut a_iter = iter.next().unwrap().trim_left_matches("a=<").trim_right_matches(">").split(",");
        Particle {
            p: (
                p_iter.next().unwrap().parse().unwrap(),
                p_iter.next().unwrap().parse().unwrap(),
                p_iter.next().unwrap().parse().unwrap(),
            ),
            v: (
                v_iter.next().unwrap().parse().unwrap(),
                v_iter.next().unwrap().parse().unwrap(),
                v_iter.next().unwrap().parse().unwrap(),
            ),
            a: (
                a_iter.next().unwrap().parse().unwrap(),
                a_iter.next().unwrap().parse().unwrap(),
                a_iter.next().unwrap().parse().unwrap(),
            ),
        }
    }
}

impl Particle {
    fn advance(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
}

const NUM_RUNS: usize = 100_000;

struct Particles(Vec<Particle>);

impl Particles {
    fn from_input(input: &str) -> Self {
        Particles(input.trim().lines().map(Particle::from_line).collect())
    }

    fn remove_dups(&mut self) {
        let mut map = HashMap::new();
        let mut curr_idx = 0;
        while curr_idx < self.0.len() {
            let particle = self.0[curr_idx];
            if let Some(idx) = map.get(&particle.p).map(|p| *p) {
                assert_ne!(curr_idx, idx);
                let mut ahead_idx = curr_idx + 1;
                while ahead_idx < self.0.len() {
                    if particle.p == self.0[ahead_idx].p {
                        self.0.remove(ahead_idx);
                    } else {
                        ahead_idx += 1;
                    }
                }
                self.0.remove(curr_idx);
                self.0.remove(idx);
                map.clear();
                curr_idx = 0;
                continue
            }
            map.insert(particle.p, curr_idx);
            curr_idx += 1;
        }
    }

    fn advance_all(&mut self) {
        for particle in self.0.iter_mut() {
            particle.advance();
        }
    }
}

fn run(input: &str) {
    let mut particles = Particles::from_input(input);
    for _ in 0..NUM_RUNS {
        particles.remove_dups();
        particles.advance_all();
    }
    println!("solution: {}", particles.0.len());
}

fn main() {
    run(include_str!("../input"));
}

#[test]
fn test_from_line() {
    assert_eq!(
        Particle {
            p: (3458, 4134, -3451),
            v: (-12, -132, 64),
            a: (-9, -2, 5),
        },
        Particle::from_line("p=<3458,4134,-3451>, v=<-12,-132,64>, a=<-9,-2,5>")
    )
}
