//! If you have any problem understanding the code,
//! just debug and print the Ground step by step in fn traversal 😊

use lazy_static::lazy_static;
// use log::debug;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    ops::RangeInclusive,
    str::FromStr,
};

use aoc2018::{load, AoCError, Result};

lazy_static! {
    static ref RE1: Regex = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
}

lazy_static! {
    static ref RE2: Regex = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
}

fn main() -> Result<()> {
    let s = load(17);

    let mut ground = Ground::new();

    for line in s.lines() {
        ground.add_clay(line.parse()?);
    }

    ground.traversal();
    part1(&ground);
    part2(&ground);

    Ok(())
}

fn part1(ground: &Ground) {
    println!("part1: {}", ground.total_water());
}

fn part2(ground: &Ground) {
    println!("part2: {}", ground.rested_water());
}

#[derive(Debug)]
struct Ground {
    spring: Coondinate,
    clay: HashSet<Coondinate>,
    water: HashMap<Coondinate, Water>,
    rested: HashSet<Coondinate>,
    min: Coondinate,
    max: Coondinate,
}

impl Ground {
    fn new() -> Self {
        Self {
            spring: Coondinate { x: 500, y: 0 },
            clay: HashSet::new(),
            water: HashMap::new(),
            rested: HashSet::new(),
            min: Coondinate {
                x: 500,
                y: u64::MAX,
            },
            max: Coondinate { x: 0, y: 0 },
        }
    }

    fn add_clay(&mut self, scan: ClayScan) {
        for x in scan.clone().x {
            for y in scan.clone().y {
                self.clay.insert(Coondinate { x, y });
                self.min.x = self.min.x.min(x);
                self.max.x = self.max.x.max(x);
                self.min.y = self.min.y.min(y);
                self.max.y = self.max.y.max(y);
            }
        }
    }

    fn traversal(&mut self) {
        // avoid corner case
        self.min.x -= 1;
        self.max.x += 1;
        self.min.y -= 1;

        let mut queue = vec![self.spring];
        while let Some(c) = queue.last() {
            let mut down = c.to_owned();
            down.y += 1;
            if !self.min.min_include(&down) {
                queue.pop();
                queue.push(down);
                continue;
            } else if !self.max.max_include(&down) {
                queue.pop();
                continue;
            }

            // debug!("{}", self);

            if self.clay.contains(&down) || self.rested.contains(&down) {
                let left = self.flow_left(c);
                let right = self.flow_right(c);
                let mut c = queue.pop().unwrap();
                match (left.1, right.1) {
                    (Water::Flow, Water::Flow) => {
                        c.x = left.0;
                        queue.push(c);

                        c.x = right.0;
                        queue.push(c);

                        for x in left.0..=right.0 {
                            c.x = x;
                            self.water.insert(c, Water::Flow);
                        }
                    }
                    (Water::Flow, Water::Rest) => {
                        c.x = left.0;
                        queue.push(c);

                        for x in left.0..=right.0 {
                            c.x = x;
                            self.water.insert(c, Water::Flow);
                        }
                    }
                    (Water::Rest, Water::Flow) => {
                        c.x = right.0;
                        queue.push(c);

                        for x in left.0..=right.0 {
                            c.x = x;
                            self.water.insert(c, Water::Flow);
                        }
                    }
                    (Water::Rest, Water::Rest) => {
                        for x in left.0..=right.0 {
                            c.x = x;
                            self.water.insert(c, Water::Rest);
                            self.rested.insert(c);
                        }
                    }
                }
            } else if let Some(water) = self.water.get(&down) {
                match water {
                    Water::Flow => {
                        queue.pop();
                    }
                    _ => unreachable!(),
                }
            } else {
                self.water.insert(down, Water::Flow);
                queue.push(down);
            }
        }
        // debug!("{}", &self);
    }

    // check left bound then down bound
    // #|
    //  #
    fn flow_left(&mut self, base: &Coondinate) -> (u64, Water) {
        let mut left = *base;
        left.x -= 1;
        while self.min.min_include(&left) {
            if self.clay.contains(&left) || self.rested.contains(&left) {
                return (left.x + 1, Water::Rest);
            }

            left.y += 1;
            if !self.clay.contains(&left) && !self.rested.contains(&left) {
                return (left.x, Water::Flow);
            }
            left.y -= 1;

            left.x -= 1;
        }
        unreachable!()
    }

    // check right bound then down bound
    //  |#
    //  #
    fn flow_right(&mut self, base: &Coondinate) -> (u64, Water) {
        let mut right = *base;
        right.x += 1;
        while self.max.max_include(&right) {
            if self.clay.contains(&right) || self.rested.contains(&right) {
                return (right.x - 1, Water::Rest);
            }

            right.y += 1;
            if !self.clay.contains(&right) && !self.rested.contains(&right) {
                return (right.x, Water::Flow);
            }
            right.y -= 1;

            right.x += 1;
        }
        unreachable!()
    }

    fn total_water(&self) -> u64 {
        let mut count = 0;
        for (c, _w) in self.water.iter() {
            if c.y > self.min.y {
                count += 1;
            }
        }
        count
    }

    fn rested_water(&self) -> u64 {
        let mut count = 0;
        for c in self.rested.iter() {
            if c.y > self.min.y {
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coondinate {
    x: u64,
    y: u64,
}

impl Coondinate {
    #[inline]
    fn min_include(&self, other: &Coondinate) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    fn max_include(&self, other: &Coondinate) -> bool {
        self.x >= other.x && self.y >= other.y
    }
}

#[derive(Debug)]
enum Water {
    Flow,
    Rest,
}

#[derive(Debug, Clone)]
struct ClayScan {
    x: RangeInclusive<u64>,
    y: RangeInclusive<u64>,
}

impl FromStr for ClayScan {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(caps) = RE1.captures(s) {
            let (x, y1, y2) = (
                caps[1].parse::<u64>()?,
                caps[2].parse::<u64>()?,
                caps[3].parse::<u64>()?,
            );
            Ok(ClayScan {
                x: RangeInclusive::new(x, x),
                y: RangeInclusive::new(y1, y2),
            })
        } else if let Some(caps) = RE2.captures(s) {
            let (y, x1, x2) = (
                caps[1].parse::<u64>()?,
                caps[2].parse::<u64>()?,
                caps[3].parse::<u64>()?,
            );
            Ok(ClayScan {
                x: RangeInclusive::new(x1, x2),
                y: RangeInclusive::new(y, y),
            })
        } else {
            Err(AoCError::DirtyInput)
        }
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let c = Coondinate { x, y };
                if self.clay.contains(&c) {
                    // check no conflict
                    if self.water.contains_key(&c) {
                        panic!("flow conflict")
                    }
                    if self.rested.contains(&c) {
                        panic!("rest conflict")
                    }
                    write!(f, "#")?;
                } else if let Some(water) = self.water.get(&c) {
                    match water {
                        Water::Flow => {
                            if self.rested.contains(&c) {
                                panic!("conflict")
                            }
                            write!(f, "|")?;
                        }
                        Water::Rest => {
                            //still check conflict
                            if !self.rested.contains(&c) {
                                panic!("conflict")
                            }
                            write!(f, "~")?;
                        }
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
