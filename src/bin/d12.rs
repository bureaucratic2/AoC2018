use aoc2018::{load, AoCError};
use regex::Regex;
use slice_deque::SliceDeque;
use std::collections::HashMap;

fn main() -> Result<(), AoCError> {
    let s = load(12);
    let mut lines = s.lines();
    let re = Regex::new(r"initial state: ([.|#]+)")?;
    // VecDeque can not be sliced so SliceDeque is used.
    let state = re.captures(lines.next().unwrap()).unwrap()[1]
        .chars()
        .collect::<SliceDeque<_>>();
    let mut state = vec![state.clone(), state];
    lines.next().unwrap();

    let mut rules = HashMap::new();
    let mut preds = Vec::new();
    let mut cons = Vec::new();
    for line in lines {
        let mut rule = line.split(" => ");
        preds.push(rule.next().unwrap().chars().collect::<Vec<_>>());
        cons.push(rule.next().unwrap().chars().next().unwrap());
    }

    for (pred, con) in preds.iter().zip(cons.iter()) {
        rules.insert(pred.as_slice(), *con);
    }

    part1(state.clone(), &rules);
    part2(&mut state, &rules);

    Ok(())
}

fn part1(mut state: Vec<SliceDeque<char>>, rules: &HashMap<&[char], char>) {
    let mut offset = 0;

    for _ in 0..3 {
        state[0].push_front('.');
        state[1].push_front('.');
    }
    offset += 3;
    for _ in 0..3 {
        state[0].push_back('.');
        state[1].push_back('.');
    }

    let mut cur = 1;
    let iterations: i64 = 20;
    for _ in 0..iterations {
        cur = 1 - cur;
        offset += next_generation(&mut state, rules, cur);
    }

    cur = 1 - cur;
    let sum: i64 = state[cur]
        .iter()
        .enumerate()
        .filter(|&elem| elem.1 == &'#')
        .map(|elem| elem.0 as i64 - offset)
        .sum();
    println!("part1: {}", sum);
}

/// state will be stable in a few generations, find out stable state and
/// skip 50000000000 iterations.
fn part2(state: &mut Vec<SliceDeque<char>>, rules: &HashMap<&[char], char>) {
    let mut offset = 0;

    for _ in 0..3 {
        state[0].push_front('.');
        state[1].push_front('.');
    }
    offset += 3;
    for _ in 0..3 {
        state[0].push_back('.');
        state[1].push_back('.');
    }

    let mut cur = 1;
    let mut iterations: i64 = 50000000000;
    let mut prev = 0;
    let mut diff = 0;
    let mut count = 0;

    while iterations > 0 {
        cur = 1 - cur;
        offset += next_generation(state, rules, cur);

        let sum: i64 = state[1 - cur]
            .iter()
            .enumerate()
            .filter(|&elem| elem.1 == &'#')
            .map(|elem| elem.0 as i64 - offset)
            .sum();

        if diff == sum - prev {
            count += 1;
        } else {
            diff = sum - prev;
            count = 0;
        }
        println!("{}th generation diff {}", 50000000000 - iterations, diff);
        if count == 10 {
            print!(
                "part2: found stable point at {}th generation, ",
                50000000000 - iterations
            );
            break;
        }

        prev = sum;
        iterations -= 1;
    }

    println!("res {}", prev + diff * iterations);
}

fn next_generation(
    state: &mut Vec<SliceDeque<char>>,
    rules: &HashMap<&[char], char>,
    cur: usize,
) -> i64 {
    let mut offset = 0;
    while state[cur][2] == '#' {
        state[0].push_front('.');
        state[1].push_front('.');
        offset += 1;
    }
    while state[cur][state[cur].len() - 3] == '#' {
        state[0].push_back('.');
        state[1].push_back('.');
    }

    let dir = 1 - cur;
    for i in 0..state[cur].len() - 4 {
        state[dir][i + 2] = *rules.get(&state[cur][i..i + 5]).unwrap_or(&'.');
    }

    offset
}

#[test]
fn char_hash() {
    let s = load(12);
    let mut lines = s.lines();
    lines.next().unwrap();
    lines.next().unwrap();

    let mut rules = HashMap::new();
    let mut preds = Vec::new();
    let mut cons = Vec::new();
    for line in lines {
        let mut rule = line.split(" => ");
        preds.push(rule.next().unwrap().chars().collect::<Vec<_>>());
        cons.push(rule.next().unwrap().chars().next().unwrap());
    }

    for (pred, con) in preds.iter().zip(cons.iter()) {
        let pred = pred.as_slice();
        rules.insert(pred, *con);
    }

    for (pred, con) in preds.iter().zip(cons.iter()) {
        let pred = pred.as_slice();
        println!("{:?} {}", rules.get(pred), con);
    }
}
