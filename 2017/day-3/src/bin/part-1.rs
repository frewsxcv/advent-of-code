// Position (x, y) relative to the center
type Pos = (isize, isize);

// Direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
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

fn main() {
    println!("solution: {}", calculate_steps(347991));
}

fn calculate_steps(square: usize) -> usize {
    // Number of squares to place in current row/col
    let mut rowcol_num = 1;
    // How many have been placed in current row/col
    let mut curr_rowcol_num = 0;
    // Iterator yielding directions indefinitely in the correct ordering
    let mut dir_iter = DIR_ORDER.iter().cycle();
    // Current direction
    let mut dir = dir_iter.next().unwrap();
    // Current position
    let mut pos: Pos = (0, 0);

    for _ in 1..square {
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

    (pos.0.abs() + pos.1.abs()) as usize
}

#[test]
fn test_examples() {
    assert_eq!(0, calculate_steps(1));
    assert_eq!(3, calculate_steps(12));
    assert_eq!(2, calculate_steps(23));
    assert_eq!(31, calculate_steps(1024));
}
