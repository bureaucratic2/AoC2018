use petgraph::{algo::astar, graph::NodeIndex, Graph};
use std::str::FromStr;

use aoc2018::{load, AoCError, Result};

fn main() -> Result<()> {
    let s = load(22);

    let mut cave = s.parse::<Cave>()?;
    cave.initial_risk();
    cave.risk_level();
    cave.shortest();

    Ok(())
}

#[derive(Debug, Default)]
struct Cave {
    mouth: Coordinate,
    target: Coordinate,
    depth: usize,
    erosion: Vec<Vec<usize>>,
    regions: Vec<Vec<Region>>,
}

impl Cave {
    fn initial_risk(&mut self) {
        let depth = self.depth;
        for (x, region) in self.erosion[0].iter_mut().enumerate() {
            *region = index2level(depth, x * 16807);
        }
        for y in 0..self.erosion.len() {
            self.erosion[y][0] = index2level(depth, y * 48271);
        }

        for y in 1..self.erosion.len() {
            for x in 1..self.erosion[0].len() {
                self.erosion[y][x] =
                    index2level(depth, self.erosion[y - 1][x] * self.erosion[y][x - 1]);
            }
        }
        self.erosion[self.target.y][self.target.x] = self.erosion[self.mouth.y][self.mouth.x];

        for (y, row) in self.regions.iter_mut().enumerate() {
            for (x, region) in row.iter_mut().enumerate() {
                *region = (self.erosion[y][x] % 3).into();
            }
        }
    }

    fn risk_level(&self) {
        let risk_level = self
            .regions
            .iter()
            .take(self.target.y + 1)
            .map(|row| {
                row.iter()
                    .take(self.target.x + 1)
                    .map(|region| {
                        let risk: usize = (*region).into();
                        risk
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        println!("part1: {}", risk_level);
    }

    /// Each slot in the cave is treated as two vertices
    /// corresponding to 2 equipments.
    ///
    /// rocky: 0(climbing gear), 1(torch)
    ///
    /// wet: 0, 2(neither)
    ///
    /// narrow: 1, 2
    fn shortest(&self) {
        let mut paths = Graph::<(), usize>::new();
        let ylen = self.regions.len();
        let xlen = self.regions[0].len();
        let mut index = vec![vec![[NodeIndex::new(0); 3]; xlen]; ylen];

        for (y, row) in self.regions.iter().enumerate() {
            for (x, region) in row.iter().enumerate() {
                match region {
                    Region::Rocky => {
                        index[y][x][0] = paths.add_node(());
                        index[y][x][1] = paths.add_node(());
                    }
                    Region::Wet => {
                        index[y][x][0] = paths.add_node(());
                        index[y][x][2] = paths.add_node(());
                    }
                    Region::Narrow => {
                        index[y][x][1] = paths.add_node(());
                        index[y][x][2] = paths.add_node(());
                    }
                }
            }
        }

        for (y, row) in self.regions.iter().enumerate() {
            for (x, region) in row.iter().enumerate() {
                let equipments = match region {
                    Region::Rocky => [0, 1],
                    Region::Wet => [0, 2],
                    Region::Narrow => [1, 2],
                };
                for (nx, ny) in self.around((x, y)) {
                    let n_equipments = match self.regions[ny][nx] {
                        Region::Rocky => [0, 1],
                        Region::Wet => [0, 2],
                        Region::Narrow => [1, 2],
                    };
                    for e in 0..2 {
                        for ne in 0..2 {
                            let weight = if equipments[e] == n_equipments[ne] {
                                // allow to move directly, cost 1 min.
                                1
                            } else if equipments[e] == n_equipments[1 - ne]
                                || equipments[1 - e] == n_equipments[ne]
                            {
                                // move directly to next slot and change the equipment or
                                // change the equipment, then move to next slot
                                // cost 1 + 7 || 7 + 1 mins
                                8
                            } else {
                                // change the equipment, then move to next slot, finnally change the equipment
                                // cost 7 + 1 + 7 mins
                                15
                            };
                            paths.add_edge(
                                index[y][x][equipments[e]],
                                index[ny][nx][n_equipments[ne]],
                                weight,
                            );
                        }
                    }
                }
            }
        }

        // 1 means equip torch at the mouth, move to target with climbing gear
        // so the answer should add 7.
        let res1 = astar(
            &paths,
            index[0][0][1],
            |end| end == index[self.target.y][self.target.x][0],
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap()
        .0 + 7;

        // move to target with torch equipped
        let res2 = astar(
            &paths,
            index[0][0][1],
            |end| end == index[self.target.y][self.target.x][1],
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap()
        .0;
        println!("part2: {}", res1.min(res2));
    }

    fn around(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let mut surrounded = vec![];

        if p.0 > 0 {
            surrounded.push((p.0 - 1, p.1));
        }
        if p.1 > 0 {
            surrounded.push((p.0, p.1 - 1));
        }
        if p.0 < self.regions[0].len() - 1 {
            surrounded.push((p.0 + 1, p.1));
        }
        if p.1 < self.regions.len() - 1 {
            surrounded.push((p.0, p.1 + 1));
        }
        surrounded
    }
}

#[inline]
fn index2level(depth: usize, index: usize) -> usize {
    (index + depth) % 20183
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.split(',');
        let (x, y) = (s.next().unwrap(), s.next().unwrap());
        Ok(Coordinate {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl FromStr for Cave {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut cave = Cave::default();
        for s in s.lines() {
            let mut s = s.split(": ");
            let (mark, data) = (s.next().unwrap(), s.next().unwrap());
            match mark {
                "depth" => cave.depth = data.parse()?,

                "target" => {
                    cave.target = data.parse()?;
                    // +80 to make sure we can find the shortest path
                    cave.erosion = vec![vec![0; cave.target.x + 80]; cave.target.y + 80];
                    cave.regions =
                        vec![vec![Region::Rocky; cave.target.x + 80]; cave.target.y + 80];
                }
                _ => unreachable!(),
            }
        }
        Ok(cave)
    }
}

#[derive(Debug, Clone, Copy)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for Region {
    fn from(risk: usize) -> Self {
        match risk {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

impl From<Region> for usize {
    fn from(region: Region) -> Self {
        match region {
            Region::Rocky => 0,
            Region::Wet => 1,
            Region::Narrow => 2,
        }
    }
}
