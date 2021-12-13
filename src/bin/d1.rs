use aoc2018::load;

use std::collections::HashSet;

fn main() {
    let s = load(1);

    let oscillating = s
        .split('\n')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    part1(&oscillating);
    part2(&oscillating);
}

fn part1(oscillating: &[i64]) {
    println!("part1: {}", oscillating.iter().sum::<i64>());
}

fn part2(oscillating: &[i64]) {
    let mut res = HashSet::new();
    let mut cur = 0;
    res.insert(cur);
    loop {
        for change in oscillating.iter() {
            cur += *change;
            if !res.insert(cur) {
                println!("part2: {}", cur);
                return;
            }
        }
    }
}
