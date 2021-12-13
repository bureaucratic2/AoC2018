use aoc2018::{load, Result};

fn main() -> Result<()> {
    let s = load(14);
    let input = s.parse::<usize>()?;

    part1(input);
    part2(input);

    Ok(())
}

fn part1(input: usize) {
    let mut recipes = Recipes::new();

    while recipes.scores.len() < input + 10 {
        recipes.step();
    }

    print!("part1: ");
    for _ in recipes
        .scores
        .iter()
        .skip(input)
        .take(10)
        .map(|score| print!("{}", score))
    {}
    println!();
}

fn part2(mut input: usize) {
    let mut scores = Vec::new();
    while input > 0 {
        scores.push(input % 10);
        input /= 10;
    }
    scores.reverse();

    let mut recipes = Recipes::new();
    let ends_at;
    loop {
        if recipes.scores.ends_with(&scores) {
            ends_at = recipes.scores.len() - scores.len();
            break;
        } else if recipes.scores[..recipes.scores.len() - 1].ends_with(&scores) {
            ends_at = recipes.scores.len() - scores.len() - 1;
            break;
        }

        recipes.step();
    }

    println!("part2: {}", ends_at);
}

struct Recipes {
    elves: Vec<usize>,
    scores: Vec<usize>,
}

impl Recipes {
    fn new() -> Self {
        Self {
            elves: vec![0, 1],
            scores: vec![3, 7],
        }
    }

    fn step(&mut self) {
        let next = self
            .elves
            .iter()
            .map(|cur| self.scores[*cur])
            .sum::<usize>();

        if next / 10 > 0 {
            self.scores.push(next / 10);
        }
        self.scores.push(next % 10);

        for elf in self.elves.iter_mut() {
            *elf += 1 + self.scores[*elf];
            *elf %= self.scores.len();
        }
    }
}
