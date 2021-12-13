use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::Dfs,
};
use std::{collections::HashSet, str::FromStr};

use aoc2018::{load, AoCError, Result};

fn main() -> Result<()> {
    let s = load(25);

    part1(&s)?;

    Ok(())
}

fn part1(s: &str) -> Result<()> {
    let constellation = s.parse::<Constellation>()?;
    let res = constellation.search();
    println!("part1: {}", res);
    Ok(())
}

/// Constellation is just undirected graph,
/// where two points' distance <= 3 means there is an edge between them.
struct Constellation {
    inner: UnGraph<Coordinate, ()>,
    map: Vec<(NodeIndex, Coordinate)>,
}

impl Constellation {
    /// use DFS to search connected component's number
    fn search(&self) -> u64 {
        let mut visited = HashSet::new();
        let mut dfs = Dfs::empty(&self.inner);
        let mut count = 0;

        for (ix, _) in self.map.iter() {
            if !visited.contains(ix) {
                count += 1;
                visited.insert(*ix);
                dfs.move_to(*ix);
                while let Some(node) = dfs.next(&self.inner) {
                    visited.insert(node);
                }
            }
        }

        count
    }
}

impl FromStr for Constellation {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut graph = UnGraph::<Coordinate, ()>::default();
        let mut v = vec![];
        for s in s.lines() {
            let c = s.parse::<Coordinate>()?;
            let ix = graph.add_node(c);
            v.push((ix, c));
        }

        for i in 0..v.len() {
            for j in i + 1..v.len() {
                if v[i].1.distance(&v[j].1) <= 3 {
                    graph.add_edge(v[i].0, v[j].0, ());
                }
            }
        }

        Ok(Constellation {
            inner: graph,
            map: v,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> i64 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }
}

impl FromStr for Coordinate {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.split(',');

        Ok(Coordinate {
            x: s.next().ok_or(AoCError::DirtyInput)?.parse()?,
            y: s.next().ok_or(AoCError::DirtyInput)?.parse()?,
            z: s.next().ok_or(AoCError::DirtyInput)?.parse()?,
            w: s.next().ok_or(AoCError::DirtyInput)?.parse()?,
        })
    }
}
