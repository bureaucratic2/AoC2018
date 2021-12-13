use aoc2018::{load, AoCError, Result};
use std::{
    fmt::{self, Display},
    mem,
    str::FromStr,
};

fn main() -> Result<()> {
    let s = load(18);

    let mut area = s.parse::<Area>()?;

    part1(&mut area);
    part2(&mut area);
    Ok(())
}

fn part1(area: &mut Area) {
    for _ in 0..10 {
        area.step();
    }

    println!("part1: {}", area.wood() * area.yard());
}

/// It is easy to know that a cycle appear after hundreds of steps
/// so we can use this to skip most of the steps.
fn part2(area: &mut Area) {
    // warm up to enter the cycle
    for _ in 0..1000 {
        area.step();
    }

    let mut last = 1000000000 - 1010;
    let sentinel = (area.wood(), area.yard());
    let mut cycle = 0;

    while last > 0 {
        last -= 1;
        cycle += 1;
        area.step();

        // cycle found
        if sentinel == (area.wood(), area.yard()) {
            break;
        }
    }
    // skip
    last %= cycle;

    for _ in 0..last {
        area.step();
    }
    println!("part2: {}", area.wood() * area.yard());
}

struct Area {
    used: Vec<Vec<Acre>>,
    backup: Vec<Vec<Acre>>,
}

impl Area {
    fn step(&mut self) {
        for (y, row) in self.used.iter().enumerate() {
            for (x, acre) in row.iter().enumerate() {
                match acre {
                    Acre::Open => {
                        let count = self.around(x, y, 0, |count, n| {
                            if n == &Acre::Wooded {
                                count + 1
                            } else {
                                count
                            }
                        });
                        self.backup[y][x] = if count >= 3 {
                            Acre::Wooded
                        } else {
                            self.used[y][x]
                        };
                    }
                    Acre::Wooded => {
                        let count = self.around(x, y, 0, |count, n| {
                            if n == &Acre::Lumberyard {
                                count + 1
                            } else {
                                count
                            }
                        });

                        self.backup[y][x] = if count >= 3 {
                            Acre::Lumberyard
                        } else {
                            self.used[y][x]
                        };
                    }
                    Acre::Lumberyard => {
                        let (tree, yard) =
                            self.around(x, y, (false, false), |(tree, yard), n| match n {
                                Acre::Open => (tree, yard),
                                Acre::Wooded => (true, yard),
                                Acre::Lumberyard => (tree, true),
                            });

                        self.backup[y][x] = if tree && yard {
                            Acre::Lumberyard
                        } else {
                            Acre::Open
                        };
                    }
                }
            }
        }

        mem::swap(&mut self.used, &mut self.backup);
    }

    fn around<T>(&self, ox: usize, oy: usize, init: T, mut f: impl FnMut(T, &Acre) -> T) -> T {
        let mut res = init;
        for y in oy.saturating_sub(1)..=oy + 1 {
            for x in ox.saturating_sub(1)..=ox + 1 {
                if x == ox && y == oy {
                    continue;
                }
                if x >= self.width() || y >= self.length() {
                    continue;
                }

                res = f(res, &self.used[y][x]);
            }
        }
        res
    }

    fn wood(&self) -> u64 {
        let mut count = 0;
        for row in self.used.iter() {
            for acre in row.iter() {
                if acre == &Acre::Wooded {
                    count += 1;
                }
            }
        }
        count
    }

    fn yard(&self) -> u64 {
        let mut count = 0;
        for row in self.used.iter() {
            for acre in row.iter() {
                if acre == &Acre::Lumberyard {
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    fn width(&self) -> usize {
        self.used[0].len()
    }

    #[inline]
    fn length(&self) -> usize {
        self.used.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Acre {
    Open,
    Wooded,
    Lumberyard,
}

impl Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Acre::Open => '.',
                Acre::Wooded => '|',
                Acre::Lumberyard => '#',
            }
        )
    }
}

impl FromStr for Area {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut area = vec![];
        for s in s.lines() {
            let mut row = vec![];
            for ch in s.chars() {
                row.push(match ch {
                    '.' => Acre::Open,
                    '|' => Acre::Wooded,
                    '#' => Acre::Lumberyard,
                    _ => unreachable!(),
                });
            }
            area.push(row);
        }
        Ok(Area {
            used: area.clone(),
            backup: area.clone(),
        })
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.used.iter() {
            for acre in row.iter() {
                write!(f, "{}", acre)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
