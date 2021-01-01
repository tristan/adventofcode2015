use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Register {
    A,
    B
}

impl From<&str> for Register {
    fn from(s: &str) -> Register {
        match s {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("unknown register")
        }
    }
}

#[derive(Debug)]
enum Offset {
    Pos(usize),
    Neg(usize)
}

impl From<&str> for Offset {
    fn from(s: &str) -> Offset {
        let val = s[1..].parse::<usize>().unwrap();
        if &s[..1] == "-" {
            Offset::Neg(val)
        } else {
            Offset::Pos(val)
        }
    }
}

impl Offset {
    fn adjust(&self, rhs: usize) -> usize {
        match self {
            Offset::Pos(offset) => {
                rhs + offset
            },
            Offset::Neg(offset) => {
                rhs.wrapping_sub(*offset)
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl From<&str> for Op {
    fn from(line: &str) -> Op {
        match &line[..3] {
            "hlf" => Op::Hlf(line[4..].into()),
            "tpl" => Op::Tpl(line[4..].into()),
            "inc" => Op::Inc(line[4..].into()),
            "jmp" => Op::Jmp(line[4..].into()),
            "jie" => Op::Jie(line[4..5].into(), line[7..].into()),
            "jio" => Op::Jio(line[4..5].into(), line[7..].into()),
            _ => panic!("invalid instruction")
        }
    }
}

struct State {
    a: usize,
    b: usize,
    counter: usize
}

impl State {
    fn new(a: usize, b: usize) -> State {
        State { a, b, counter: 0 }
    }

    fn execute(&mut self, program: &[Op]) {
        while self.counter < program.len() {
            match &program[self.counter] {
                Op::Hlf(Register::A) => {
                    self.a /= 2;
                    self.counter += 1;
                },
                Op::Hlf(Register::B) => {
                    self.b /= 2;
                    self.counter += 1;
                },
                Op::Tpl(Register::A) => {
                    self.a += self.a + self.a;
                    self.counter += 1;
                },
                Op::Tpl(Register::B) => {
                    self.b += self.b + self.b;
                    self.counter += 1;
                },
                Op::Inc(Register::A) => {
                    self.a += 1;
                    self.counter += 1;
                },
                Op::Inc(Register::B) => {
                    self.b += 1;
                    self.counter += 1;
                },
                Op::Jmp(offset) => {
                    self.counter = offset.adjust(self.counter);
                },
                Op::Jie(r, offset) => {
                    if match r {
                        Register::A => self.a % 2 == 0,
                        Register::B => self.b % 2 == 0
                    } {
                        self.counter = offset.adjust(self.counter);
                    } else {
                        self.counter += 1;
                    }
                },
                Op::Jio(r, offset) => {
                    if match r {
                        Register::A => self.a == 1,
                        Register::B => self.b == 1
                    } {
                        self.counter = offset.adjust(self.counter);
                    } else {
                        self.counter += 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let program = BufReader::new(File::open("day_23_input.txt").unwrap())
        .lines().filter_map(Result::ok)
        .map(|line| line.as_str().into())
        .collect::<Vec<Op>>();
    let mut state = State::new(0, 0);
    state.execute(&program);
    println!("part1: {}", state.b);
    let mut state = State::new(1, 0);
    state.execute(&program);
    println!("part2: {}", state.b);
}
