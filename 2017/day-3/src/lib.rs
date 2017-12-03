// Position (x, y) relative to the center
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pos(isize, isize);

impl Pos {
    fn neighbors(self) -> [Pos; 8] {
        [
            Pos(self.0, self.1 + 1),
            Pos(self.0 + 1, self.1 + 1),
            Pos(self.0 + 1, self.1),
            Pos(self.0 + 1, self.1 - 1),
            Pos(self.0, self.1 - 1),
            Pos(self.0 - 1, self.1 - 1),
            Pos(self.0 - 1, self.1),
            Pos(self.0 - 1, self.1 + 1),
        ]
    }
}

// Direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn apply(self, pos: &mut Pos) {
        match self {
            Dir::Up => pos.1 += 1,
            Dir::Right => pos.0 += 1,
            Dir::Down => pos.1 -= 1,
            Dir::Left => pos.0 -= 1,
        }
    }
}

static DIR_ORDER: [Dir; 4] = [
    Dir::Right,
    Dir::Up,
    Dir::Left,
    Dir::Down,
];

#[derive(Clone, Copy, Debug)]
pub struct Square {
    square_num: usize,
    pos: Pos,
}

impl Square {
    pub fn from_square_num(square_num: usize) -> Self {
        // Number of squares to place in current row/col
        let mut rowcol_num = 1;
        // How many have been placed in current row/col
        let mut curr_rowcol_num = 0;
        // Iterator yielding directions indefinitely in the correct ordering
        let mut dir_iter = DIR_ORDER.iter().cycle();
        // Current direction
        let mut dir = dir_iter.next().unwrap();
        // Current position
        let mut pos = Pos(0, 0);

        for _ in 1..square_num {
            dir.apply(&mut pos);
            curr_rowcol_num += 1;
            if curr_rowcol_num == rowcol_num {
                if *dir == Dir::Up || *dir == Dir::Down {
                    rowcol_num += 1;
                }
                curr_rowcol_num = 0;
                dir = dir_iter.next().unwrap();
            }
        }

        Square {
            square_num: square_num,
            pos: Pos(pos.0, pos.1),
        }
    }

    pub fn steps(self) -> usize {
        (self.pos.0.abs() + self.pos.1.abs()) as usize
    }

    pub fn value(self) -> usize {
        if self.pos.0 == 0 && self.pos.1 == 0 {
            1
        } else {
            self.neighbors().into_iter().map(|n| n.value()).sum()
        }
    }

    pub fn neighbors(self) -> Vec<Square> {
        let pos_neighbors = self.pos.neighbors();
        (1..self.square_num)
            .map(|n| Square::from_square_num(n))
            .filter(|s| pos_neighbors.iter().find(|p| **p == s.pos).is_some())
            .collect()
    }
}

#[test]
fn test_steps_examples() {
    assert_eq!(0, Square::from_square_num(1).steps());
    assert_eq!(3, Square::from_square_num(12).steps());
    assert_eq!(2, Square::from_square_num(23).steps());
    assert_eq!(31, Square::from_square_num(1024).steps());
}

#[test]
fn test_values() {
    assert_eq!(1, Square::from_square_num(1).value());
    assert_eq!(1, Square::from_square_num(2).value());
    assert_eq!(2, Square::from_square_num(3).value());
    assert_eq!(10, Square::from_square_num(6).value());
}
