#![feature(int_abs_diff)]

// use log::debug;
use std::{
    cell::RefCell,
    cmp,
    collections::{HashSet, VecDeque},
    fmt::{self, Debug},
    rc::Rc,
};

use aoc2018::{load, Result};

type Map = Vec<Vec<Slot>>;

fn main() -> Result<()> {
    // setup_logger().unwrap();
    let s = load(15);

    part1(s.clone());
    part2(s);
    Ok(())
}

fn part1(s: String) {
    let mut battle = Battle::new(s, 3, true);

    loop {
        let (_, success, res) = battle.round();
        if success {
            println!("part1: {}", res);
            break;
        }
    }
}

fn part2(s: String) {
    let mut attack = 4;
    'outer: loop {
        let mut battle = Battle::new(s.clone(), attack, false);
        loop {
            let (race, success, res) = battle.round();
            if success {
                match race {
                    Race::Elf => {
                        println!("part2: {}", res);
                        break 'outer;
                    }
                    Race::Goblin => break,
                }
            }
        }
        attack += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.1.cmp(&other.1) {
            cmp::Ordering::Equal => self.0.cmp(&other.0),
            res => res,
        }
    }
}

struct Unit {
    race: Race,
    hitpoint: i32,
    attack: i32,
    loc: Point,
    map: Rc<RefCell<Map>>,
}

impl Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        write!(f, "{:?}({}) at {:?}", self.race, self.hitpoint, self.loc)?;
        Ok(())
    }
}

impl Unit {
    fn new(race: Race, loc: Point, map: Rc<RefCell<Map>>, attack: i32) -> Self {
        Self {
            race,
            hitpoint: 200,
            attack,
            loc,
            map,
        }
    }

    fn around(&self, p: Option<Point>, slot: Slot) -> Vec<Point> {
        let map = self.map.borrow();

        // Map is surrounded by walls so we have confidence that
        // it would not be out-of-bound.
        let mut surrounded = vec![];
        let Point(x, y) = p.unwrap_or(self.loc);
        if map[y - 1][x] == slot {
            surrounded.push(Point(x, y - 1));
        }
        if map[y][x - 1] == slot {
            surrounded.push(Point(x - 1, y));
        }
        if map[y][x + 1] == slot {
            surrounded.push(Point(x + 1, y));
        }
        if map[y + 1][x] == slot {
            surrounded.push(Point(x, y + 1));
        }
        surrounded
    }

    fn unit_move(&self, targets: HashSet<Point>) -> Option<Point> {
        // caculate distance under the condition that no blocks exist
        // todo: distance
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut res = vec![];
        let mut shortest = 0;
        queue.push_back((0, vec![self.loc]));
        while !queue.is_empty() {
            let (dist, path) = queue.pop_front().unwrap();
            if shortest > 0 && path.len() > shortest {
                break;
            }
            let node = path.last().unwrap();
            if targets.contains(node) {
                shortest = path.len();
                res.push(path);
                continue;
            }
            if visited.contains(node) {
                continue;
            }
            visited.insert(*node);
            for neighbor in self.around(Some(*node), Slot::Cavern) {
                if visited.contains(&neighbor) {
                    continue;
                }
                let mut path = path.clone();
                path.push(neighbor);
                queue.push_back((dist + 1, path));
            }
        }

        if shortest > 0 {
            res.sort_unstable_by_key(|a| a.len());
            let next_loc = res
                .iter()
                .filter(|&path| path.len() == shortest)
                .map(|path| path[1])
                .min()
                .unwrap();

            Some(next_loc)
        } else {
            None
        }
    }
}

struct Battle {
    map: Rc<RefCell<Map>>,
    units: Vec<Unit>,
    elves: usize,
    allow_dead: bool,
    goblins: usize,
    round: i32,
}

impl Battle {
    fn new(s: String, attack: i32, allow_dead: bool) -> Self {
        let origin = Rc::new(RefCell::new(Vec::new()));

        let r_handle = Rc::clone(&origin);

        let mut units = vec![];
        let mut map = origin.borrow_mut();
        let mut elves = 0;
        let mut goblins = 0;
        for (y, row) in s.lines().enumerate() {
            map.push(Vec::new());
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    '#' => map[y].push(Slot::Wall),
                    '.' => map[y].push(Slot::Cavern),
                    'G' => {
                        map[y].push(Slot::Occupied(Race::Goblin));
                        units.push(Unit::new(
                            Race::Goblin,
                            Point(x, y),
                            Rc::clone(&r_handle),
                            3,
                        ));
                        goblins += 1;
                    }
                    'E' => {
                        map[y].push(Slot::Occupied(Race::Elf));
                        units.push(Unit::new(
                            Race::Elf,
                            Point(x, y),
                            Rc::clone(&r_handle),
                            attack,
                        ));
                        elves += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
        drop(map);

        Self {
            map: origin,
            units,
            elves,
            goblins,
            round: 0,
            allow_dead,
        }
    }

    fn round(&mut self) -> (Race, bool, i32) {
        // sort the reading order
        self.units.sort_unstable_by(|a, b| a.loc.cmp(&b.loc));
        let elves = self.elves;

        // debug!("{:?}", self);
        // traverse all units and attck-move
        let mut full_round = true;
        let units = self.units.len();
        for index in 0..units {
            if self.units[index].hitpoint <= 0 {
                continue;
            }
            if !self.attack(index) {
                let targets = self.targets(&self.units[index].race);
                if let Some(next_loc) = self.units[index].unit_move(targets) {
                    update_map(&self.map, next_loc, Slot::Occupied(self.units[index].race));
                    update_map(&self.map, self.units[index].loc, Slot::Cavern);
                    self.units[index].loc = next_loc;
                    self.attack(index);
                }
            }
            if index != units && (self.elves == 0 || self.goblins == 0) {
                full_round = false;
                break;
            }
        }

        if full_round {
            self.round += 1;
        }

        if !self.allow_dead && elves != self.elves {
            return (Race::Goblin, true, 0);
        }

        // delete dead
        self.units.retain(|unit| unit.hitpoint > 0);

        // check end condition
        if self.elves == 0 || self.goblins == 0 {
            // debug!("Endgame");
            // debug!("{:?}", self);
            let hps = self.units.iter().map(|unit| unit.hitpoint).sum::<i32>();
            let race = self.units[0].race;
            return (race, true, hps * self.round);
        }

        (Race::Elf, false, 0)
    }

    fn targets(&self, race: &Race) -> HashSet<Point> {
        let mut targets = HashSet::new();

        for unit in self
            .units
            .iter()
            .filter(|&unit| &unit.race != race && unit.hitpoint > 0)
        {
            for target in unit.around(None, Slot::Cavern) {
                targets.insert(target);
            }
        }

        targets
    }

    fn attack(&mut self, index: usize) -> bool {
        let (enemy, attack) = (
            match self.units[index].race {
                Race::Elf => Race::Goblin,
                Race::Goblin => Race::Elf,
            },
            self.units[index].attack,
        );
        let attack_targets = self.units[index].around(None, Slot::Occupied(enemy));
        if !attack_targets.is_empty() {
            let mut targets = Vec::with_capacity(attack_targets.len());
            'outer: for target in attack_targets {
                for (index, unit) in self.units.iter().enumerate() {
                    if unit.loc == target {
                        targets.push((unit.hitpoint, target, index));
                        continue 'outer;
                    }
                }
            }
            targets.sort_unstable();
            let target = targets[0];
            let unit = &mut self.units[target.2];
            unit.hitpoint -= attack;

            // update map
            if unit.hitpoint <= 0 {
                match enemy {
                    Race::Elf => self.elves -= 1,
                    Race::Goblin => self.goblins -= 1,
                }
                update_map(&self.map, unit.loc, Slot::Cavern);
            }

            return true;
        }
        false
    }
}

impl Debug for Battle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        let map = self.map.borrow();
        if self.round == 0 {
            writeln!(f, "Initially:")?;
        } else {
            writeln!(f, "After {} round:", self.round)?;
        }

        // before print debug information, caller should promise
        // that units is sorted.
        let mut index = 0;

        for row in map.iter() {
            let mut units = Vec::new();
            let s = row
                .iter()
                .map(|slot| match slot {
                    Slot::Wall => '#',
                    Slot::Cavern => '.',
                    Slot::Occupied(race) => match race {
                        Race::Elf => {
                            units.push(('E', self.units[index].hitpoint));
                            index += 1;
                            'E'
                        }
                        Race::Goblin => {
                            units.push(('G', self.units[index].hitpoint));
                            index += 1;
                            'G'
                        }
                    },
                })
                .collect::<String>();
            write!(f, "{}", s)?;
            if !units.is_empty() {
                write!(f, "   ")?;
                let mut iter = units.into_iter();
                let unit = iter.next().unwrap();
                write!(f, "{}({})", unit.0, unit.1)?;
                for unit in iter {
                    write!(f, ", ")?;
                    write!(f, "{}({})", unit.0, unit.1)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn update_map(map: &Rc<RefCell<Map>>, p: Point, slot: Slot) {
    let Point(x, y) = p;
    map.borrow_mut()[y][x] = slot;
}

#[derive(Debug, PartialEq, Eq)]
enum Slot {
    Wall,
    Cavern,
    Occupied(Race),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Race {
    Elf,
    Goblin,
}
