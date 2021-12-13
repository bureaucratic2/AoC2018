//! this program is used to factor a number stored in R3 and store the sum of all factors in R0
//! in a really stupid manner shown below
//! R3 = (biiiiiiiig number)
//! R2 = 1
//! R5 = 1
//! loop R5 < R3
//!     loop R2 < R3
//!         R4 = R2 * R5
//!         if R4 != R3
//!             R2 += 1
//!         else
//!             R0 += R5
//!     R2 = 1
//!     R5 += 1        
//!

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
};

use aoc2018::{load, AoCError, Result};

lazy_static! {
    // name to index map
    static ref MAP: HashMap<String, usize> = {
        let names = "addr, addi, mulr, muli, banr, bani, borr, bori, seti, setr, gtir, gtri, gtrr, eqir, eqri, eqrr";
        let mut map = HashMap::new();

        for (index, name) in names.split(", ").enumerate() {
            map.insert(name.to_string(), index);
        }

        map
    };
    // index to name map
    static ref REV: HashMap<usize, String> = {
        let names = "addr, addi, mulr, muli, banr, bani, borr, bori, seti, setr, gtir, gtri, gtrr, eqir, eqri, eqrr";
        let mut map = HashMap::new();

        for (index, name) in names.split(", ").enumerate() {
            map.insert(index, name.to_string());
        }

        map
    };
}

const OPS: [OP; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, seti, setr, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

type OP = fn(&[usize], &mut [usize]);

fn main() -> Result<()> {
    let s = load(19);

    part1(&s);
    part2(&s);

    Ok(())
}

fn part1(s: &str) {
    let mut assembler = Assembler::load(s).unwrap();
    assembler.exec(u64::MAX);
    println!("part1: {:?}", assembler.regs[0]);
}

fn part2(s: &str) {
    // fast
    let mut assembler = Assembler::load(s).unwrap();
    assembler.regs[0] = 1;
    assembler.exec(100);

    let big_number = assembler.regs[3];
    let mut sum = 0;
    for i in 1..=big_number {
        if big_number % i == 0 {
            sum += i;
        }
    }
    println!("part2: {}", sum);
}

#[derive(Debug, Default)]
struct Assembler {
    instructions: Vec<Instruction>,
    origin: Vec<String>, // for debug
    regs: [usize; 6],
    ip: usize,
}

impl Assembler {
    fn load(s: &str) -> Result<Self> {
        let mut assembler = Assembler::default();
        for origin in s.lines() {
            let instruction = origin.parse()?;
            match instruction {
                Instruction::IP(reg) => assembler.ip = reg,
                i => {
                    assembler.instructions.push(i);
                    assembler.origin.push(origin.to_string())
                }
            }
        }

        Ok(assembler)
    }

    fn exec(&mut self, mut limit: u64) {
        let ip = self.ip;
        // let mut count = 0;
        while let Some(instruction) = self.instructions.get(self.regs[ip]) {
            match instruction {
                Instruction::OPCode(i) => {
                    OPS.get(i[0]).unwrap()(i, &mut self.regs);
                    // debug!(
                    //     "\n{}[{}, {}, {}, {}, {}, {}]",
                    //     instruction,
                    //     self.regs[0],
                    //     self.regs[1],
                    //     self.regs[2],
                    //     self.regs[3],
                    //     self.regs[4],
                    //     self.regs[5]
                    // );
                }
                _ => unreachable!(),
            }
            self.regs[ip] += 1;
            limit -= 1;
            if limit == 0 {
                break;
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    OPCode([usize; 4]),
    IP(usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::OPCode(i) => {
                if let Some(name) = REV.get(&i[0]) {
                    let start = &name[0..2];
                    let end = &name[2..];
                    match end {
                        "rr" => match start {
                            "gt" => return writeln!(f, "R{} = R{} > R{}", i[3], i[1], i[2]),

                            "eq" => return writeln!(f, "R{} = R{} == R{}", i[3], i[1], i[2]),
                            _ => {}
                        },
                        "ir" => match start {
                            "gt" => return writeln!(f, "R{} = {} > R{}", i[3], i[1], i[2]),

                            "eq" => return writeln!(f, "R{} = {} == R{}", i[3], i[1], i[2]),
                            _ => {}
                        },
                        "ri" => match start {
                            "gt" => return writeln!(f, "R{} = R{} > {}", i[3], i[1], i[2]),

                            "eq" => return writeln!(f, "R{} = R{} == {}", i[3], i[1], i[2]),
                            _ => {}
                        },
                        _ => {}
                    }

                    let start = &name[0..3];
                    let end = &name[3..];
                    match end {
                        "r" => {
                            let op = match start {
                                "add" => '+',
                                "mul" => '*',
                                "ban" => '&',
                                "bor" => '|',
                                "set" => return writeln!(f, "R{} = R{}", i[3], i[1]),
                                _ => unreachable!(),
                            };

                            if i[3] == i[1] {
                                return writeln!(f, "R{} {}= R{}", i[3], op, i[2]);
                            } else if i[3] == i[2] {
                                return writeln!(f, "R{} {}= R{}", i[3], op, i[1]);
                            } else {
                                return writeln!(f, "R{} = R{} {} R{}", i[3], i[1], op, i[2]);
                            }
                        }
                        "i" => {
                            let op = match start {
                                "add" => '+',
                                "mul" => '*',
                                "ban" => '&',
                                "bor" => '|',
                                "set" => return writeln!(f, "R{} = {}", i[3], i[1]),
                                _ => unreachable!(),
                            };

                            if i[3] == i[1] {
                                return writeln!(f, "R{} {}= {}", i[3], op, i[2]);
                            } else if i[3] == i[2] {
                                return writeln!(f, "R{} {}= {}", i[3], op, i[1]);
                            } else {
                                return writeln!(f, "R{} = R{} {} {}", i[3], i[1], op, i[2]);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Instruction::IP(_) => unreachable!(),
        }
        Err(fmt::Error)
    }
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("#ip") {
            let reg = s.trim_start_matches("#ip ");
            Ok(Instruction::IP(reg.parse()?))
        } else {
            let opcode = s.split(' ');
            let mut buf = vec![];
            for elem in opcode {
                buf.push(elem);
            }

            if buf.len() != 4 {
                return Err(AoCError::DirtyInput);
            }

            if MAP.get(buf[0]).is_none() {
                return Err(AoCError::DirtyInput);
            }

            Ok(Instruction::OPCode([
                *MAP.get(buf[0]).unwrap(),
                buf[1].parse()?,
                buf[2].parse()?,
                buf[3].parse()?,
            ]))
        }
    }
}

// Implement all instruction
fn addr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] + register[instruction[2]];
}

fn addi(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] + instruction[2];
}

fn mulr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] * register[instruction[2]];
}

fn muli(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] * instruction[2];
}

fn banr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] & register[instruction[2]];
}

fn bani(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] & instruction[2];
}

fn borr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] | register[instruction[2]];
}

fn bori(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]] | instruction[2];
}

fn setr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = register[instruction[1]];
}

fn seti(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = instruction[1];
}

fn gtir(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if instruction[1] > register[instruction[2]] {
        1
    } else {
        0
    };
}

fn gtri(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if register[instruction[1]] > instruction[2] {
        1
    } else {
        0
    };
}

fn gtrr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if register[instruction[1]] > register[instruction[2]] {
        1
    } else {
        0
    };
}

fn eqir(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if instruction[1] == register[instruction[2]] {
        1
    } else {
        0
    };
}

fn eqri(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if register[instruction[1]] == instruction[2] {
        1
    } else {
        0
    };
}

fn eqrr(instruction: &[usize], register: &mut [usize]) {
    register[instruction[3]] = if register[instruction[1]] == register[instruction[2]] {
        1
    } else {
        0
    };
}
