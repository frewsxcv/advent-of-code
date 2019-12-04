use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

struct Movement {
    direction: Direction,
    amount: i32,
}

impl Movement {
    fn from_str(s: &str) -> Movement {
        let mut chars = s.chars();
        let direction = Direction::from_char(chars.next().unwrap());
        let amount = chars.collect::<String>().parse::<i32>().unwrap();
        Movement { direction, amount }
    }
}

type Path = Vec<Movement>;
type Coord = (i32, i32); // (x, y)

fn path_from_input_line(input_line: &str) -> Path {
    input_line.split(",").map(Movement::from_str).collect()
}

fn visited_coords_for_path(mut path: Path) -> HashSet<Coord> {
    let mut curr = (0, 0);
    let mut visited = HashSet::new();
    loop {
        if path.is_empty() {
            return visited;
        }
        match path[0].direction {
            Direction::Up => curr.1 += 1,
            Direction::Down => curr.1 -= 1,
            Direction::Left => curr.0 -= 1,
            Direction::Right => curr.0 += 1,
        }
        path[0].amount -= 1;
        if path[0].amount == 0 {
            path.remove(0);
        }
        visited.insert(curr);
    }
}

fn paths_from_input() -> [Path; 2] {
    let mut input_lines = INPUT.lines();
    [
        path_from_input_line(input_lines.next().unwrap()),
        path_from_input_line(input_lines.next().unwrap()),
    ]
}

fn main() {
    let [wire1_path, wire2_path] = paths_from_input();

    let wire1_visited = visited_coords_for_path(wire1_path);
    let wire2_visited = visited_coords_for_path(wire2_path);

    let shortest = wire1_visited
        .intersection(&wire2_visited)
        .map(|i| i.0 + i.1)
        .min()
        .unwrap();

    println!("part 1: {}", shortest);
}
