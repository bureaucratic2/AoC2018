use aoc2018::{load, Result};
use regex::Regex;

fn main() -> Result<()> {
    let s = load(3);

    let mut fabric = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut claims = vec![];

    for claim in s.lines() {
        let caps = re.captures(claim).unwrap();
        let start = (caps[2].parse::<usize>()?, caps[3].parse::<usize>()?);
        let end = (
            start.0 + caps[4].parse::<usize>()?,
            start.1 + caps[5].parse::<usize>()?,
        );

        for row in fabric.iter_mut().take(end.1).skip(start.1) {
            for index in row.iter_mut().take(end.0).skip(start.0) {
                *index += 1;
            }
        }

        claims.push(Claim { start, end });
    }

    part1(&fabric);
    part2(&fabric, &claims);

    Ok(())
}

fn part1(fabric: &[Vec<i32>]) {
    let mut overlap = 0;

    for row in fabric.iter() {
        for index in row.iter() {
            if index > &1 {
                overlap += 1;
            }
        }
    }
    println!("part1: {}", overlap);
}

fn part2(fabric: &[Vec<i32>], claims: &Vec<Claim>) {
    'loop1: for (idx, claim) in claims.iter().enumerate() {
        for row in fabric.iter().take(claim.end.1).skip(claim.start.1) {
            for index in row.iter().take(claim.end.0).skip(claim.start.0) {
                if index != &1 {
                    continue 'loop1;
                }
            }
        }
        println!("part2: {}", idx + 1);
        return;
    }
}

struct Claim {
    start: (usize, usize),
    end: (usize, usize),
}
