use std::collections::{HashMap, VecDeque};

use aoc2018::{load, Result};

use regex_syntax::{
    hir::{Hir, HirKind, Literal},
    ParserBuilder,
};

use petgraph::graph::{NodeIndex, UnGraph};

fn main() -> Result<()> {
    let s = load(20);

    let hir = ParserBuilder::new()
        .nest_limit(1000)
        .build()
        .parse(s.trim())
        .unwrap();
    let mut dists = HashMap::new();

    part1(hir, &mut dists);
    part2(&dists);

    Ok(())
}

fn part1(hir: Hir, dists: &mut HashMap<NodeIndex, usize>) {
    let mut rooms = UnGraph::<Coordinate, ()>::default();
    let c = Coordinate { x: 0, y: 0 };
    let root = rooms.add_node(c);
    let mut exist = HashMap::new();
    exist.insert(c, root);

    create_map(&hir, &mut rooms, c, &mut exist);
    bfs(&rooms, dists, root);
    println!("part1: {}", dists.values().max().unwrap());
}

fn part2(dists: &HashMap<NodeIndex, usize>) {
    println!(
        "part2: {}",
        dists
            .values()
            .filter(|&dist| dist >= &1000)
            .map(|_| 1)
            .sum::<usize>()
    );
}

fn create_map(
    hir: &Hir,
    rooms: &mut UnGraph<Coordinate, ()>,
    parent: Coordinate,
    exist: &mut HashMap<Coordinate, NodeIndex>,
) -> Coordinate {
    match hir.kind() {
        HirKind::Literal(Literal::Unicode(ch)) => {
            let next = parent.mv(*ch);
            let node = if let Some(node) = exist.get(&next) {
                *node
            } else {
                let idx = rooms.add_node(next);
                exist.insert(next, idx);
                idx
            };
            rooms.update_edge(
                *exist
                    .get(&parent)
                    .unwrap_or_else(|| panic!("{:?} don't exist", parent)),
                node,
                (),
            );
            next
        }
        HirKind::Group(ref group) => create_map(&group.hir, rooms, parent, exist),
        HirKind::Concat(a) => {
            let mut next = parent;
            for hir in a.iter() {
                next = create_map(hir, rooms, next, exist);
            }
            next
        }
        HirKind::Alternation(a) => {
            for hir in a {
                create_map(hir, rooms, parent, exist);
            }
            parent
        }
        _ => parent,
    }
}

/// Use BFS to find the shortest distance from room to origin room.
fn bfs(rooms: &UnGraph<Coordinate, ()>, dists: &mut HashMap<NodeIndex, usize>, root: NodeIndex) {
    let mut queue = VecDeque::new();
    queue.push_back((root, 0));
    while let Some((node, dist)) = queue.pop_front() {
        if let Some(origin) = dists.get_mut(&node) {
            if *origin <= dist {
                continue;
            } else {
                *origin = dist;
            }
        } else {
            dists.insert(node, dist);
        }

        for neighbor in rooms.neighbors(node) {
            queue.push_back((neighbor, dist + 1));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn mv(&self, ch: char) -> Self {
        let mut c = *self;
        match ch {
            'N' => c.y -= 1,
            'S' => c.y += 1,
            'W' => c.x -= 1,
            'E' => c.x += 1,
            _ => unreachable!(),
        }
        c
    }
}
