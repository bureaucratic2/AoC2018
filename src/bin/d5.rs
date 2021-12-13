use aoc2018::load;

const DIFF: u8 = b'a' - b'A';

fn main() {
    let s = load(5);

    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if !stack.is_empty() {
            let last = stack.last().unwrap();
            if react(*last, ch) {
                stack.pop();
                continue;
            }
        }
        stack.push(ch);
    }

    part1(&stack);
    part2(&stack);
}

fn part1(simplified: &[char]) {
    println!("part1: {}", simplified.len());
}

fn part2(simplified: &[char]) {
    let shortest = (0..26)
        .map(|ch| ((ch + b'a') as char, (ch + b'A') as char))
        .map(|(ref low, ref high)| {
            let mut stack = Vec::new();
            for ch in simplified.iter() {
                if ch == low || ch == high {
                    continue;
                }
                if !stack.is_empty() {
                    let last = stack.last().unwrap();
                    if react(*last, *ch) {
                        stack.pop();
                        continue;
                    }
                }
                stack.push(*ch);
            }
            stack.len()
        })
        .min()
        .unwrap();
    println!("part2: {}", shortest);
}

#[inline]
fn react(ch1: char, ch2: char) -> bool {
    ch1 as u8 + DIFF == ch2 as u8 || ch2 as u8 + DIFF == ch1 as u8
}
