const INPUT: &str = include_str!("input.txt");

type Opcode = usize;

const ADD_OPCODE: Opcode = 1;
const MULT_OPCODE: Opcode = 2;
const DONE_OPCODE: Opcode = 99;

struct Program(Vec<Opcode>);

impl Program {
    fn from_input() -> Self {
        Program(INPUT.split(",").map(|n| n.parse().unwrap()).collect())
    }

    fn restore_gravity_assist_program(&mut self) {
        self.0[1] = 12;
        self.0[2] = 2;
    }

    fn run(&mut self) {
        let mut idx = 0;
        loop {
            match self.0[idx] {
                ADD_OPCODE => self.execute_add(idx),
                MULT_OPCODE => self.execute_mult(idx),
                DONE_OPCODE => break,
                _ => unreachable!(),
            }
            idx += 4;
        }
    }

    fn execute_add(&mut self, idx: usize) {
        debug_assert!(self.0[idx] == ADD_OPCODE);
        let src1_idx = self.0[idx + 1];
        let src2_idx = self.0[idx + 2];
        let dst_idx = self.0[idx + 3];
        self.0[dst_idx] = self.0[src1_idx] + self.0[src2_idx];
    }

    fn execute_mult(&mut self, idx: usize) {
        debug_assert!(self.0[idx] == MULT_OPCODE);
        let src1_idx = self.0[idx + 1];
        let src2_idx = self.0[idx + 2];
        let dst_idx = self.0[idx + 3];
        self.0[dst_idx] = self.0[src1_idx] * self.0[src2_idx];
    }
}

fn main() {
    let mut program = Program::from_input();
    program.restore_gravity_assist_program();
    program.run();
    println!("part 1: {}", program.0[0]);
}
