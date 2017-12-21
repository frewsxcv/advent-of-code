use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next_coord(self, prev_coord: Coord) -> Coord {
        match self {
            Dir::Up => (prev_coord.0, prev_coord.1 - 1),
            Dir::Down => (prev_coord.0, prev_coord.1 + 1),
            Dir::Left => (prev_coord.0 - 1, prev_coord.1),
            Dir::Right => (prev_coord.0 + 1, prev_coord.1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PathItem {
    Corner,
    HorizLine,
    VertLine,
    Letter(char),
}

impl PathItem {
    fn from_char(char_: char) -> Option<Self> {
        match char_ {
            '+' => Some(PathItem::Corner),
            '-' => Some(PathItem::HorizLine),
            '|' => Some(PathItem::VertLine),
            c if c.is_alphabetic() => Some(PathItem::Letter(c)),
            c if c.is_whitespace() => None,
            _ => unreachable!(),
        }
    }
}

// (x, y)
type Coord = (usize, usize);

struct Executor {
    map: HashMap<Coord, PathItem>,
    curr_dir: Dir,
    curr_coord: Coord,
}

impl Executor {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut first_coord = None;
       for (y, line) in input.trim_left_matches("\n").lines().enumerate() {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if let Some(pi) = PathItem::from_char(c) {
                        Some((x, pi))
                    } else {
                        None
                    }
                })
                .for_each(|(x, path_item)| {
                    if y == 0 && path_item == PathItem::VertLine {
                        first_coord = Some((x, y))
                    }
                    map.insert((x, y), path_item);
                })
        }
        Executor {
            map,
            curr_dir: Dir::Down,
            curr_coord: first_coord.unwrap(),
        }
    }

    fn next_dir(&self) -> Dir {
        if let Some(pi) = self.map.get(&(self.curr_coord.0, self.curr_coord.1 - 1)) {
            if *pi != PathItem::Corner {
                return Dir::Up;
            }
        }
        if let Some(pi) = self.map.get(&(self.curr_coord.0, self.curr_coord.1 + 1)) {
            if *pi != PathItem::Corner {
                return Dir::Down;
            }
        }
        if let Some(pi) = self.map.get(&(self.curr_coord.0 + 1, self.curr_coord.1)) {
            if *pi != PathItem::Corner {
                return Dir::Right;
            }
        }
        if let Some(pi) = self.map.get(&(self.curr_coord.0 - 1, self.curr_coord.1)) {
            if *pi != PathItem::Corner {
                return Dir::Left;
            }
        }
        unreachable!()
    }

    fn run(&mut self) {
        let mut count = 0;
        const LAST_LETTER: char = 'B';
        loop {
            count += 1;
            self.curr_coord = self.curr_dir.next_coord(self.curr_coord);
            match self.map.get(&self.curr_coord) {
                Some(&PathItem::Corner) => self.curr_dir = self.next_dir(),
                Some(&PathItem::Letter(LAST_LETTER)) => break,
                _ => (),
            };
            self.map.remove(&self.curr_coord);
        }
        println!("solution: {}", count + 1);
    }
}

fn main() {
    Executor::new(include_str!("../input")).run();
}
