use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Op {
    Snd(char),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(char, Val),
}

impl Op {
    fn from_str(s: &str) -> Op {
        let mut words = s.trim().split(" ");
        let op = words.next().unwrap();
        let register = words.next().unwrap().chars().next().unwrap();
        match op {
            "snd" => Op::Snd(register),
            "set" => Op::Set(register, Val::from_str(words.next().unwrap())),
            "add" => Op::Add(register, Val::from_str(words.next().unwrap())),
            "mul" => Op::Mul(register, Val::from_str(words.next().unwrap())),
            "mod" => Op::Mod(register, Val::from_str(words.next().unwrap())),
            "rcv" => Op::Rcv(register),
            "jgz" => Op::Jgz(register, Val::from_str(words.next().unwrap())),
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
            Val::Reg(r) => *self.0.entry(r).or_insert(0),
        }
    }
}

struct Executor {
    regs: Registers,
    ops: Vec<Op>,
    curr: usize,
    last_sound: Option<i64>,
}

impl Executor {
    fn new(input: &str) -> Executor {
        Executor {
            regs: Registers::default(),
            ops: input.trim().lines().map(|l| Op::from_str(l)).collect(),
            curr: 0,
            last_sound: None,
        }
    }

    fn run(&mut self) -> i64 {
        loop {
            match self.ops[self.curr] {
                Op::Snd(reg) => {
                    let num = self.regs.0.entry(reg).or_insert(0);
                    self.last_sound = Some(*num);
                }
                Op::Set(reg, val) => {
                    let num = self.regs.resolve(val);
                    self.regs.0.insert(reg, num);
                }
                Op::Add(reg, val) => {
                    let num = self.regs.resolve(val);
                    *self.regs.0.entry(reg).or_insert(0) += num;
                }
                Op::Mul(reg, val) => {
                    let num = self.regs.resolve(val);
                    *self.regs.0.entry(reg).or_insert(0) *= num;
                }
                Op::Mod(reg, val) => {
                    let num = self.regs.resolve(val);
                    *self.regs.0.entry(reg).or_insert(0) %= num;
                }
                Op::Rcv(reg) => {
                    let num = self.regs.0.entry(reg).or_insert(0);
                    if *num != 0 {
                        return self.last_sound.unwrap();
                    }
                }
                Op::Jgz(reg, val) => {
                    let num = *self.regs.0.entry(reg).or_insert(0);
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
    println!("solution: {}", Executor::new(include_str!("../input")).run());
}
