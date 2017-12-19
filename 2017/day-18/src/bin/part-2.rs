use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
            Val::Reg(r @ 'a'...'z') => *self.0.entry(r).or_insert(0),
            Val::Reg(_) => unreachable!(),
        }
    }
}

struct Executor {
    regs: Registers,
    ops: Vec<Op>,
    curr: usize,
    send: mpsc::Sender<i64>,
    recv: mpsc::Receiver<i64>,
}

impl Executor {
    fn new(input: &str, program_id: i64, send: mpsc::Sender<i64>, recv: mpsc::Receiver<i64>) -> Executor {
        let mut regs = Registers::default();
        regs.0.insert('p', program_id);
        Executor {
            regs: regs,
            ops: input.trim().lines().map(|l| Op::from_str(l)).collect(),
            curr: 0,
            send: send,
            recv: recv,
        }
    }

    fn run(&mut self) -> i64 {
        let mut counter = 0;
        loop {
            match self.ops[self.curr] {
                Op::Snd(reg) => {
                    counter += 1;
                    let num = match reg {
                        'a'...'z' => self.regs.0.entry(reg).or_insert(0),
                        _ => unreachable!(),
                    };
                    self.send.send(*num).unwrap();
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
                    match self.recv.recv_timeout(Duration::from_secs(1)) {
                        Ok(v) => {
                            self.regs.0.insert(reg, v);
                        },
                        Err(_) => return counter,
                    };
                }
                Op::Jgz(reg, val) => {
                    // `reg` can be a number, which is weird
                    let num = match reg {
                        '0' => 0,
                        '1'...'9' => 1,
                        'a'...'z' => *self.regs.0.entry(reg).or_insert(0),
                        _ => unreachable!(),
                    };
                    if num > 0 {
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
    let (send_a, recv_a) = mpsc::channel();
    let (send_b, recv_b) = mpsc::channel();

    let mut executor_a = Executor::new(include_str!("../input"), 0, send_a, recv_b);
    let mut executor_b = Executor::new(include_str!("../input"), 1, send_b, recv_a);

    let thread_a = thread::spawn(move || {
        let _ = executor_a.run();
    });
    let thread_b = thread::spawn(move || {
        println!("solution: {}", executor_b.run());
    });
    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
