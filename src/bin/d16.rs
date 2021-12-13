use std::collections::HashSet;

use aoc2018::{load, Result};

const BEFORE_PREFIX: usize = "Before: [".len();
const AFTER_PREFIX: usize = "After:  [".len();

const OPS: [OP; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, seti, setr, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

type OP = fn(&[usize], &mut [usize]);

fn main() -> Result<()> {
    let s = load(16);
    let mut s = s.split("\n\n\n");

    let map = part1(s.next().unwrap());
    part2(s.next().unwrap(), map);
    Ok(())
}

fn part1(s: &str) -> Vec<HashSet<usize>> {
    let mut map = vec![HashSet::new(); OPS.len()];

    let mut lines = s.lines();
    let mut count = 0;
    loop {
        let before = lines.next().unwrap();
        let instruction = lines.next().unwrap();
        let after = lines.next().unwrap();

        let before = pares_register(before, BEFORE_PREFIX);
        let instruction = parse_instruction(instruction);
        let after = pares_register(after, AFTER_PREFIX);

        let mut opcodes = 0;
        let mut register = before.clone();
        let mut set = HashSet::new();
        for (index, op) in OPS.iter().enumerate() {
            op(&instruction, &mut register);
            if register == after {
                set.insert(index);
                opcodes += 1;
            }
            // reset register
            register.clone_from_slice(&before);
        }

        let opcode = &mut map[instruction[0]];
        *opcode = if opcode.is_empty() {
            set
        } else {
            opcode.intersection(&set).copied().collect()
        };

        if opcodes >= 3 {
            count += 1;
        }

        if lines.next().is_none() {
            break;
        }
    }

    println!("part1: {}", count);
    map
}

fn part2(s: &str, mut map: Vec<HashSet<usize>>) {
    let mut decided = map
        .iter()
        .filter(|&set| set.len() == 1)
        .map(|set| *set.iter().next().unwrap())
        .collect::<HashSet<_>>();

    loop {
        let mut flag = false;
        let mut new_decided = HashSet::new();

        for set in map.iter_mut() {
            if set.len() > 1 {
                for elem in decided.iter() {
                    set.remove(elem);
                    if set.len() == 1 {
                        new_decided.insert(*set.iter().next().unwrap());
                    }
                    flag = true;
                }
            }
        }
        if !flag {
            break;
        }
        decided.extend(new_decided.into_iter());
    }

    let map = map
        .into_iter()
        .map(|set| *set.iter().next().unwrap())
        .collect::<Vec<_>>();

    let mut register = vec![0; 4];
    for instruction in s.lines() {
        if instruction.is_empty() {
            continue;
        }
        let instruction = instruction
            .split(' ')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        exec(&instruction, &map, &mut register);
    }
    println!("part2: {}", register[0]);
}

fn pares_register(s: &str, prefix: usize) -> Vec<usize> {
    s[prefix..s.len() - 1]
        .split(", ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_instruction(s: &str) -> Vec<usize> {
    s.split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
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

fn exec(instruction: &[usize], map: &[usize], register: &mut [usize]) {
    let index = map[instruction[0]];
    let op = OPS[index];

    op(instruction, register);
}

#[test]
fn vec_cmp() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    assert!(v1 == v2);
}
