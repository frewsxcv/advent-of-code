// some parts shamefully taken from https://www.reddit.com/r/adventofcode/comments/7l78eb/2017_day_21_solutions/drk7mpw/

#[derive(Clone, Debug, PartialEq)]
struct Pattern(Vec<Vec<u8>>);

impl Pattern {
    fn init() -> Pattern {
        Pattern(vec![
            vec![b'.', b'#', b'.'],
            vec![b'.', b'.', b'#'],
            vec![b'#', b'#', b'#'],
        ])
    }

    fn from_str(s: &str) -> Pattern {
        Pattern(
            s.trim()
                .split("/")
                .map(|s| s.as_bytes().to_owned())
                .collect()
        )
    }

    fn new(size: usize) -> Pattern {
        let mut row = Vec::with_capacity(size);
        row.resize(size, b'.');
        let mut cells = Vec::with_capacity(size);
        cells.resize(size, row);
        Pattern(cells)
    }

    fn assemble_from(parts: Vec<Pattern>) -> Pattern {
        let num_subgrids_wide = (parts.len() as f64).sqrt() as usize;
        assert_eq!(num_subgrids_wide * num_subgrids_wide, parts.len());
        let subgrid_size = parts[0].size();

        let mut rv = Pattern::new(subgrid_size * num_subgrids_wide);

        for (idx, subgrid) in parts.iter().enumerate() {
            let gx = idx % num_subgrids_wide;
            let gy = idx / num_subgrids_wide;
            for x in 0..subgrid_size {
                for y in 0..subgrid_size {
                    rv.set((x + gx * subgrid_size, y + gy * subgrid_size), subgrid.get((x, y)));
                }
            }
        }

        rv
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.0[y][x]
    }

    fn set(&mut self, (x, y): (usize, usize), val: u8) {
        self.0[y][x] = val;
    }

    fn rotate(&self) -> Pattern {
        let size = self.size();
        let mut rotated = self.clone();
        for x in 0..size {
            for y in 0..size {
                rotated.set((y, size - x - 1), self.get((x, y)));
            }
        }
        rotated
    }

    fn flip(&self) -> Pattern {
        Pattern(
            self.0
                .iter()
                .map(|row| row.iter().rev().map(|c| *c).collect())
                .collect()
        )
    }

    fn variations(&self) -> Vec<Pattern> {
        let mut ret = vec![self.clone()];
        for i in 0..3 {
            let rotated = ret[i].rotate();
            ret.push(rotated);
        }
        for i in 0..4 {
            let flipped = ret[i].flip();
            ret.push(flipped);
        }
        ret
    }

    fn split(&self) -> Vec<Pattern> {
        let l = self.size();
        let m = if l % 2 == 0 {
            2
        } else if l % 3 == 0 {
            3
        } else {
            unreachable!()
        };
        let s = l / m;

        (0..(s * s))
            .map(|grid_idx| {
                let mut part = self.clone();
                let gx = grid_idx % s;
                let gy = grid_idx / s;
                for x in 0..m {
                    for y in 0..m {
                        part.set((x, y), self.get((x + gx * m, y + gy * m)));
                    }
                    part.0[x].truncate(m);
                }
                part.0.truncate(m);
                part
            })
            .collect()
    }

    fn matches(&self, pattern: &Pattern) -> bool {
        assert_eq!(self.size(), pattern.size());
        self.variations().into_iter().any(|v| v == *pattern)
    }

    fn num_pixels_on(&self) -> usize {
        self.0.iter().map(|row| row.iter().filter(|n| **n == b'#').count()).sum()
    }
}

#[derive(Debug)]
struct Rule {
    from: Pattern,
    to: Pattern,
}

impl Rule {
    fn from_line(line: &str) -> Rule {
        let mut iter = line.trim().split("=>");
        Rule {
            from: Pattern::from_str(iter.next().unwrap().trim()),
            to: Pattern::from_str(iter.next().unwrap().trim()),
        }
    }
}

#[derive(Debug)]
struct Book(Vec<Rule>);

impl Book {
    fn from_input(input: &str) -> Book {
        Book(input.trim().lines().map(Rule::from_line).collect())
    }

    fn lookup(&self, pattern: &Pattern) -> Pattern {
        for &Rule { ref from, ref to } in self.0.iter() {
            if from.size() == pattern.size() && from.matches(pattern) {
                return to.clone();
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Grid(Pattern);

impl Grid {
    fn init() -> Grid {
        Grid(Pattern::init())
    }

    fn step(&mut self, book: &Book) {
        let mut new = vec![];
        for pattern in self.0.split() {
            new.push(book.lookup(&pattern));
        }
        self.0 = Pattern::assemble_from(new);
    }
}

fn main() {
    let book = Book::from_input(include_str!("../input"));
    let mut grid = Grid::init();
    for _ in 0..5 {
        grid.step(&book);
    }
    println!("solution: {}", grid.0.num_pixels_on());
}
