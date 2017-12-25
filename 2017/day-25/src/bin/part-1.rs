// Begin in state A.
// Perform a diagnostic checksum after 12399302 steps.

// In state A:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the right.
//   If the current value is 1:
//     - Write the value 0.
//     - Move one slot to the right.
//     - Continue with state C.

// In state B:
//   If the current value is 0:
//     - Write the value 0.
//     - Move one slot to the left.
//     - Continue with state A.
//   If the current value is 1:
//     - Write the value 0.
//     - Move one slot to the right.
//     - Continue with state D.

// In state C:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state D.
//   If the current value is 1:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state A.

// In state D:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the left.
//     - Continue with state E.
//   If the current value is 1:
//     - Write the value 0.
//     - Move one slot to the left.
//     - Continue with state D.

// In state E:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state F.
//   If the current value is 1:
//     - Write the value 1.
//     - Move one slot to the left.
//     - Continue with state B.

// In state F:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state A.
//   If the current value is 1:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state E.

#[derive(Copy, Clone)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl State {
    fn run(self, index: &mut usize, buf: &mut [u8]) -> State {
        match self {
            State::A if buf[*index] == 0 => {
                buf[*index] = 1;
                *index += 1;
                State::B
            }
            State::A if buf[*index] == 1 => {
                buf[*index] = 0;
                *index += 1;
                State::C
            }
            State::B if buf[*index] == 0 => {
                buf[*index] = 0;
                *index -= 1;
                State::A
            }
            State::B if buf[*index] == 1 => {
                buf[*index] = 0;
                *index += 1;
                State::D
            }
            State::C if buf[*index] == 0 => {
                buf[*index] = 1;
                *index += 1;
                State::D
            }
            State::C if buf[*index] == 1 => {
                buf[*index] = 1;
                *index += 1;
                State::A
            }
            State::D if buf[*index] == 0 => {
                buf[*index] = 1;
                *index -= 1;
                State::E
            }
            State::D if buf[*index] == 1 => {
                buf[*index] = 0;
                *index -= 1;
                State::D
            }
            State::E if buf[*index] == 0 => {
                buf[*index] = 1;
                *index += 1;
                State::F
            }
            State::E if buf[*index] == 1 => {
                buf[*index] = 1;
                *index -= 1;
                State::B
            }
            State::F if buf[*index] == 0 => {
                buf[*index] = 1;
                *index += 1;
                State::A
            }
            State::F if buf[*index] == 1 => {
                buf[*index] = 1;
                *index += 1;
                State::E
            }
            _ => unreachable!(),
        }
    }
}

const NUM_STEPS: usize = 12_399_302;
const INIT_SIZE: usize = 12_000_000;

fn main() {
    let mut buf = vec![0; INIT_SIZE];
    let mut index = INIT_SIZE / 2;
    let mut state = State::A;
    for _ in 0..NUM_STEPS {
        state = state.run(&mut index, &mut buf);
    }
    let sum: u32 = buf.into_iter().map(|n| n as u32).sum();
    println!("solution: {}", sum);
}
