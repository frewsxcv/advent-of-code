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

const NUM_RUNS: usize = 10_000;

fn manhattan_distance(coord: (i64, i64, i64)) -> u32 {
    coord.0.abs() as u32 + coord.1.abs() as u32 + coord.2.abs() as u32
}

fn run(input: &str) {
    let mut particles = input.trim().lines().map(Particle::from_line).collect::<Vec<_>>();
    for _ in 0..NUM_RUNS {
        for particle in particles.iter_mut() {
            particle.advance();
        }
    }
    let mut min: (usize, u32) = (0, manhattan_distance(particles[0].p));
    for (i, particle) in particles.iter().enumerate() {
        if manhattan_distance(particle.p) < min.1 {
            min.0 = i;
            min.1 = manhattan_distance(particle.p);
        }
    }
    println!("solution: {}", min.0);
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
