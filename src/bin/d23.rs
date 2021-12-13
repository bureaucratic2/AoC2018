use std::str::FromStr;

use aoc2018::{load, AoCError, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
}

fn main() -> Result<()> {
    let s = load(23);
    let mut track = (0, 0);
    let mut bots = vec![];
    for (idx, s) in s.lines().enumerate() {
        let bot = s.parse::<Nanobot>()?;
        if bot.radius > track.1 {
            track.0 = idx;
            track.1 = bot.radius;
        }

        bots.push(bot);
    }

    part1(&bots, track.0);
    let cur = std::time::Instant::now();
    part2(&bots);
    println!("use {}s", cur.elapsed().as_millis() as f64 / 1000.0);

    Ok(())
}

fn part1(bots: &[Nanobot], idx: usize) {
    let strongest_bot = &bots[idx];
    let mut count = 0;
    for bot in bots.iter() {
        if bot.c.distance(&strongest_bot.c) <= strongest_bot.radius {
            count += 1;
        }
    }
    println!("part1: {}", count);
}

/// the hardest part in AoC2018, z3 is used to solve it.
///
/// Z3 is an SMT ("Satisfiability modulo theories") solver.
/// The essence of it is you can give it some variables and some constraints,
/// and it will tell you if those constraints can be satisfied,
/// and if it can, will give you a satisfying set of values for those variables.
/// It does so remarkably efficiently for what is essentially a fancier SAT solver;
/// what it means is basically you can give it the definition of a problem and it will give you an answer.
///
/// More recent versions of Z3 also allow for optimization problems -
/// you can tell it that you also want to maximize or minimize the value of certain expressions in your problem,
/// and it will try to find a set of values that satisfy the constraints and also maximize or minimize the expressions specified.
/// So here, I basically designed expressions for "number of robots in range" and "distance from zero" and
/// asked Z3 to maximize the number of robots in range and then minimize the distance from zero.
fn part2(bots: &[Nanobot]) {
    use z3::ast::Int;

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let optmizer = z3::Optimize::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    fn abs<'a, 'ctx>(ctx: &'ctx z3::Context, x: &'a Int<'ctx>) -> Int<'ctx> {
        x.le(&Int::from_i64(ctx, 0)).ite(&-x, x)
    }

    let mut in_range = Int::from_i64(&ctx, 0);
    for bot in bots {
        let dist_x = abs(&ctx, &(Int::from_i64(&ctx, bot.c.x) - &x));
        let dist_y = abs(&ctx, &(Int::from_i64(&ctx, bot.c.y) - &y));
        let dist_z = abs(&ctx, &(Int::from_i64(&ctx, bot.c.z) - &z));
        let r = Int::from_i64(&ctx, bot.radius as i64);

        let in_bot_range = (dist_x + dist_y + dist_z).le(&r);
        in_range += in_bot_range.ite(&Int::from_i64(&ctx, 1), &Int::from_i64(&ctx, 0));
    }
    optmizer.maximize(&in_range);

    let zero = Int::from_i64(&ctx, 0);
    let dist_x = abs(&ctx, &(&zero - &x));
    let dist_y = abs(&ctx, &(&zero - &y));
    let dist_z = abs(&ctx, &(&zero - &z));

    let dist = dist_x + dist_y + dist_z;
    optmizer.minimize(&dist);

    optmizer.check(&[z3::ast::Bool::from_bool(&ctx, true)]);
    let model = optmizer.get_model().unwrap();
    println!(
        "part2: distance to zero {}, bots in range {}",
        model.eval(&dist, true).unwrap().as_i64().unwrap(),
        model.eval(&in_range, true).unwrap().as_i64().unwrap()
    );
}

#[derive(Debug)]
struct Nanobot {
    c: Coordinate,
    radius: u64,
}

impl FromStr for Nanobot {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let caps = RE.captures(s).ok_or(AoCError::DirtyInput)?;
        Ok(Nanobot {
            c: Coordinate {
                x: caps[1].parse()?,
                y: caps[2].parse()?,
                z: caps[3].parse()?,
            },
            radius: caps[4].parse()?,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.y - other.y).abs()) as u64
    }
}
