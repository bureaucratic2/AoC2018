use aoc2018::load;

fn main() {
    let s = load(2);

    part1(&s);
    part2(&s);
}

fn part1(s: &str) {
    let mut times2 = 0;
    let mut times3 = 0;
    let mut record = [0u8; 26];

    for id in s.lines() {
        for ch in id.chars() {
            record[(ch as u8 - b'a') as usize] += 1;
        }

        if record.iter().any(|slot| slot == &2) {
            times2 += 1;
        }

        if record.iter().any(|slot| slot == &3) {
            times3 += 1;
        }

        for slot in record.iter_mut() {
            *slot = 0;
        }
    }
    println!("part1: {}", times2 * times3);
}

// brute force
// a faster way is use SIMD
fn part2(s: &str) {
    let ids = s.lines().collect::<Vec<&str>>();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(s) = differ(ids[i], ids[j]) {
                println!("part2: {}", s);
                return;
            }
        }
    }
    panic!("part2: no answer");
}

fn differ(s1: &str, s2: &str) -> Option<String> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut found = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found {
                return None;
            }
            found = true;
        }
    }
    Some(
        s1.chars()
            .zip(s2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect::<String>(),
    )
}
