use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Signal {
    Wire(u16),
    Value(u16)
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Signal::Value(v) => write!(f, "{}", v),
            Signal::Wire(w) => {
                if w & 0xff00 != 0 {
                    write!(f, "{}{}", (w >> 8) as u8 as char, (w & 0xff) as u8 as char)
                } else {
                    write!(f, "{}", (w & 0xff) as u8 as char)
                }
            }
        }
    }
}

impl Signal {
    fn from_str(s: &str) -> Signal {
        if s.chars().all(|c| c.is_numeric()) {
            Signal::Value(s.parse().unwrap())
        } else {
            let w = s.chars().fold(0u16, |w, c| (w << 8) + c as u16);
            Signal::Wire(w)
        }
    }

    fn value(&self) -> u16 {
        match self {
            &Signal::Wire(v) => v,
            &Signal::Value(v) => v
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    And(Signal, Signal),
    Or(Signal, Signal),
    Rshift(Signal, Signal),
    Lshift(Signal, Signal),
    Not(Signal),
    Signal(Signal),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::And(lhs, rhs) => write!(f, "{} AND {}", lhs, rhs),
            Op::Or(lhs, rhs) => write!(f, "{} OR {}", lhs, rhs),
            Op::Lshift(lhs, rhs) => write!(f, "{} LSHIFT {}", lhs, rhs),
            Op::Rshift(lhs, rhs) => write!(f, "{} RSHIFT {}", lhs, rhs),
            Op::Not(src) => write!(f, "NOT {}", src),
            Op::Signal(src) => write!(f, "{}", src),
        }
    }
}

fn process_ops(mut ops: Vec<(Op, u16)>, signals: &mut Vec<Option<u16>>) {
    while ops.len() > 0 {
        if let Some(pos) = ops.iter().position(|(op, output)| {
            match op {
                Op::Signal(Signal::Value(v)) => {
                    signals[*output as usize] = Some(*v);
                    true
                },
                Op::Signal(Signal::Wire(wire)) => if let Some(src) = &signals[*wire as usize] {
                    signals[*output as usize] = Some(*src);
                    true
                } else {
                    false
                },
                Op::Not(src) => if let Some(src) = &signals[src.value() as usize] {
                    signals[*output as usize] = Some(!src);
                    true
                } else {
                    false
                },
                Op::And(lhs, rhs) | Op::Or(lhs, rhs) | Op::Lshift(lhs, rhs) | Op::Rshift(lhs, rhs) => {
                    if let Some(lhs) = match lhs {
                        Signal::Wire(wire) => signals[*wire as usize],
                        Signal::Value(v) => Some(*v)
                    } {
                        if let Some(rhs) = match rhs {
                            Signal::Wire(wire) => signals[*wire as usize],
                            Signal::Value(v) => Some(*v)
                        } {
                            signals[*output as usize] = Some(match op {
                                Op::And(_, _) => lhs & rhs,
                                Op::Or(_, _) => lhs | rhs,
                                Op::Lshift(_, _) => lhs << rhs,
                                Op::Rshift(_, _) => lhs >> rhs,
                                _ => panic!("invalid op")
                            });
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            }
        }) {
            let (_op, _output) = ops.remove(pos);
            //println!("{} -> {}", op, Signal::Wire(output));
        } else {
            panic!("didn't find any ops to execute")
        }
    };
}

fn part1(ops: Vec<(Op, u16)>) -> u16 {
    let mut signals: Vec<Option<u16>> = vec![None; u16::MAX as usize];
    process_ops(ops, &mut signals);
    signals['a' as usize].unwrap()
}

fn part2(ops: Vec<(Op, u16)>, part1: u16) -> u16 {
    let mut signals: Vec<Option<u16>> = vec![None; u16::MAX as usize];
    signals['b' as usize] = Some(part1);
    process_ops(ops, &mut signals);
    signals['a' as usize].unwrap()
}

fn main() {
    let f = File::open("day_07_input.txt").unwrap();
    let ops = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut args = line.split(" ");
            let one = args.next().unwrap();
            let op = if one == "NOT" {
                let not = Signal::from_str(args.next().unwrap());
                args.next().unwrap(); // consume ->
                Op::Not(not)
            } else {
                let src = Signal::from_str(one);
                // expect
                //   -> wire
                //   OP wire -> wire
                let two = args.next().unwrap();
                if two == "->" {
                    Op::Signal(src)
                } else {
                    let rhs = Signal::from_str(args.next().unwrap());
                    args.next().unwrap(); // consume ->
                    match two {
                        "AND" => Op::And(src, rhs),
                        "LSHIFT" => Op::Lshift(src, rhs),
                        "RSHIFT" => Op::Rshift(src, rhs),
                        "OR" => Op::Or(src, rhs),
                        _ => panic!("invalid input: {}", line)
                    }
                }
            };
            let dest = args.next().unwrap();
            (op, Signal::from_str(dest).value())
        }).collect::<Vec<(Op, u16)>>();
    let part1 = part1(ops.clone());
    let part2 = part2(ops, part1);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
