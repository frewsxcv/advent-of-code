use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Op {
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(char, Val),
}

impl Op {
    fn from_str(s: &str) -> Op {
        let mut words = s.trim().split(" ");
        let op = words.next().unwrap();
        let register = words.next().unwrap().chars().next().unwrap();
        match op {
            "set" => Op::Set(register, Val::from_str(words.next().unwrap())),
            "sub" => Op::Sub(register, Val::from_str(words.next().unwrap())),
            "mul" => Op::Mul(register, Val::from_str(words.next().unwrap())),
            "jnz" => Op::Jnz(register, Val::from_str(words.next().unwrap())),
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Val {
    Num(i64),
    Reg(char),
}

impl Val {
    fn from_str(s: &str) -> Val {
        if s.chars().next().unwrap().is_alphabetic() {
            Val::Reg(s.chars().next().unwrap())
        } else {
            Val::Num(s.parse().unwrap())
        }
    }
}

#[derive(Default)]
struct Registers(HashMap<char, i64>);

impl Registers {
    fn resolve(&mut self, val: Val) -> i64 {
        match val {
            Val::Num(n) => n,
            Val::Reg(r @ 'a'...'z') => *self.0.entry(r).or_insert(0),
            Val::Reg(_) => unreachable!(),
        }
    }
}

struct Executor {
    regs: Registers,
    ops: Vec<Op>,
    curr: usize,
}

impl Executor {
    fn new(input: &str) -> Executor {
        let regs = Registers::default();
        Executor {
            regs: regs,
            ops: input.trim().lines().map(|l| Op::from_str(l)).collect(),
            curr: 0,
        }
    }

    fn run(&mut self) {
        let mut counter = 0;
        loop {
            if self.curr >= self.ops.len() {
                println!("solution: {}", counter);
                return;
            }
            match self.ops[self.curr] {
                Op::Set(reg, val) => {
                    let num = self.regs.resolve(val);
                    self.regs.0.insert(reg, num);
                }
                Op::Sub(reg, val) => {
                    let num = self.regs.resolve(val);
                    *self.regs.0.entry(reg).or_insert(0) -= num;
                }
                Op::Mul(reg, val) => {
                    let num = self.regs.resolve(val);
                    *self.regs.0.entry(reg).or_insert(0) *= num;
                    counter += 1;
                }
                Op::Jnz(reg, val) => {
                    // `reg` can be a number, which is weird
                    let num = match reg {
                        '0' => 0,
                        '1'...'9' => 1,
                        'a'...'z' => *self.regs.0.entry(reg).or_insert(0),
                        _ => unreachable!(),
                    };
                    if num != 0 {
                        self.curr = ((self.curr as i64) + self.regs.resolve(val)) as usize;
                        continue;
                    }
                }
            }
            self.curr += 1;
        }
    }
}

fn main() {
    let mut executor_a = Executor::new(include_str!("../input"));
    let _ = executor_a.run();
}
