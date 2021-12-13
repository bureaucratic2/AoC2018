// 2993 units each with 10747 hit points (immune to slashing; weak to cold) with an attack that does 6 radiation damage at initiative 19

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use aoc2018::{load, setup_logger, AoCError, Result};
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points ?\(?(?P<property>[a-z|;| |,]+)?\)? with an attack that does (?P<atk>\d+) (?P<type>[a-z]+) damage at initiative (?P<initiative>\d+)").unwrap();
    static ref WEAK: Regex = Regex::new(r"weak to ([a-z|,| ]+)").unwrap();
    static ref IMMUNE: Regex = Regex::new(r"immune to ([a-z|,| ]+)").unwrap();
}

fn main() -> Result<()> {
    let s = load(24);
    setup_logger()?;

    part1(&s)?;
    part2(&s)?;
    Ok(())
}

fn part1(s: &str) -> Result<()> {
    let mut battle = s.parse::<Battle>()?;
    let res = battle.round();
    println!("part1: {}", res.1);
    Ok(())
}

fn part2(s: &str) -> Result<()> {
    let battle = s.parse::<Battle>().unwrap();

    let range = find_range(s);
    // range.1 may be deadlock, +10 to make sure we can get a solution
    for boost in range.0..=range.1 + 10 {
        let mut battle = battle.clone();
        battle.boost(boost);
        let res = battle.round();
        if res.0 == Winner::ImmuneSystem {
            println!(
                "part2: find immune system win with smallest boost {}",
                boost
            );
            return Ok(());
        }
    }

    Ok(())
}

/// binary search
/// lower trcaks the largest boost that immune system fail
/// upper tracks the smallest boost that immune system win
fn find_range(s: &str) -> (i64, i64) {
    let mut lower = 0;
    let mut upper = 10000;

    let battle = s.parse::<Battle>().unwrap();
    let mut left = vec![];
    loop {
        let mut battle = battle.clone();
        battle.boost(upper);
        let res = battle.round();
        if res.0 == Winner::ImmuneSystem {
            left.push((upper, res.1));
            break;
        } else {
            lower = upper;
            upper *= 2;
        }
    }
    println!("[{}-{}]", lower, upper);

    while lower < upper {
        let mid = (lower + upper) / 2;
        let mut battle = battle.clone();
        battle.boost(mid);
        let res = battle.round();
        match res.0 {
            Winner::ImmuneSystem => {
                left.push((mid, res.1));
                upper = mid;
            }
            Winner::Infection => lower = mid + 1,
            Winner::Deadlock => return (lower, upper),
        }
        println!("[{}-{}]", lower, upper);
    }
    left.sort_unstable();
    println!("{}", left[0].1);

    (upper, lower)
}

#[derive(Debug, Clone)]
struct Battle {
    groups: [Vec<Group>; 2],
}

impl Battle {
    fn round(&mut self) -> (Winner, i64) {
        let mut count = 0;
        while !self.update() {
            self.choose_target();
            if !self.attcack() {
                // dead lock
                return (Winner::Deadlock, 0);
            }
            count += 1;
            if count % 100 == 0 {
                debug!("\n{}", self);
            }
        }
        for (idx, groups) in self.groups.iter().enumerate() {
            if !groups.is_empty() {
                return (
                    idx.into(),
                    groups.iter().map(|group| group.units).sum::<i64>(),
                );
            }
        }
        (1.into(), 0)
    }

    fn choose_target(&mut self) {
        let mut seq = vec![];
        for i in 0..2 {
            for j in 0..self.groups[i].len() {
                seq.push((i, j));
            }
        }
        seq.sort_unstable_by(|a, b| cmp(&self.groups[a.0][a.1], &self.groups[b.0][b.1]));
        for group in seq.iter().rev() {
            let cur = group.0;
            let attacker = &self.groups[cur][group.1];
            let mut targets = vec![];

            for (idx, defender) in self.groups[1 - cur].iter().enumerate() {
                if defender.targeted.is_some() {
                    continue;
                }
                let damage = damage(attacker, defender);
                if damage == 0 {
                    continue;
                }
                let last = targets.last();
                if last.is_none() {
                    targets.push((idx, damage));
                    continue;
                }
                let last = *last.unwrap();
                match damage.cmp(&last.1) {
                    Ordering::Less => {}
                    Ordering::Equal => targets.push((idx, damage)),
                    Ordering::Greater => {
                        targets.clear();
                        targets.push((idx, damage));
                    }
                }
            }

            if targets.is_empty() {
                continue;
            }
            targets.sort_unstable_by(|a, b| {
                cmp(&self.groups[1 - cur][a.0], &self.groups[1 - cur][b.0])
            });
            let target = targets.last().unwrap();
            self.groups[cur][group.1].target = Some(target.0);
            self.groups[1 - cur][target.0].targeted = Some(group.1);
        }
    }

    fn attcack(&mut self) -> bool {
        let mut flag = false;
        let mut seq = vec![];
        for i in 0..2 {
            for j in 0..self.groups[i].len() {
                seq.push((i, j));
            }
        }
        seq.sort_unstable_by(|a, b| {
            self.groups[a.0][a.1]
                .initiative
                .cmp(&self.groups[b.0][b.1].initiative)
        });

        for group in seq.iter().rev() {
            let cur = group.0;
            let attacker = &mut self.groups[cur][group.1];
            if attacker.units <= 0 || attacker.target.is_none() {
                continue;
            }
            // update ep
            attacker.ep = attacker.units * attacker.atk;
            let target = attacker.target.unwrap();
            let damage = damage(&self.groups[cur][group.1], &self.groups[1 - cur][target]);
            let dead = damage / self.groups[1 - cur][target].hp;
            // if nobody die, this battle is in deadlock, break
            if dead > 0 {
                flag = true;
            }

            self.groups[1 - cur][target].units -= dead;
        }
        flag
    }

    fn update(&mut self) -> bool {
        let mut flag = false;
        for i in 0..2 {
            self.groups[i].retain(|group| group.units > 0);
            if self.groups[i].is_empty() {
                flag = true;
            }
            for group in self.groups[i].iter_mut() {
                group.target = None;
                group.targeted = None;
                group.ep = group.units * group.atk;
            }
        }
        flag
    }

    fn boost(&mut self, boost: i64) {
        for group in self.groups[0].iter_mut() {
            group.atk += boost;
        }
    }
}

impl Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Immune system: ")?;
        for group in self.groups[0].iter() {
            writeln!(f, "{}", group)?;
        }
        writeln!(f, "Infection: ")?;
        for group in self.groups[1].iter() {
            writeln!(f, "{}", group)?;
        }
        Ok(())
    }
}

fn damage(attacker: &Group, defender: &Group) -> i64 {
    if defender.immunes.contains(&attacker.atk_type) {
        0
    } else if defender.weaks.contains(&attacker.atk_type) {
        2 * attacker.ep
    } else {
        attacker.ep
    }
}

impl FromStr for Battle {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut immune_system = vec![];
        let mut infection = vec![];
        let mut p = &mut immune_system;
        for s in s.lines() {
            if s.starts_with("Immune") {
                p = &mut immune_system;
                continue;
            }
            if s.starts_with("Infection:") {
                p = &mut infection;
                continue;
            }
            if s.is_empty() {
                continue;
            } else {
                p.push(s.parse::<Group>()?);
            }
        }
        Ok(Battle {
            groups: [immune_system, infection],
        })
    }
}

#[derive(Debug)]
struct Group {
    units: i64,
    hp: i64,
    weaks: HashSet<Property>,
    immunes: HashSet<Property>,
    atk: i64,
    atk_type: Property,
    initiative: u64,
    ep: i64,

    target: Option<usize>,
    targeted: Option<usize>,
}

fn cmp(a: &Group, b: &Group) -> Ordering {
    if a.ep == b.ep {
        a.initiative.cmp(&b.initiative)
    } else {
        a.ep.cmp(&b.ep)
    }
}

impl FromStr for Group {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(caps) = RE.captures(s) {
            let units = caps["units"].parse()?;
            let hp = caps["hp"].parse()?;
            let atk = caps["atk"].parse()?;
            let atk_type = caps["type"].parse()?;
            let initiative = caps["initiative"].parse()?;

            let mut weaks = HashSet::new();
            let mut immunes = HashSet::new();
            if let Some(properties) = &caps.name("property") {
                for property in properties.as_str().split("; ") {
                    if let Some(weak) = WEAK.captures(property) {
                        for weak in weak[1].split(", ") {
                            weaks.insert(weak.parse::<Property>()?);
                        }
                    }
                    if let Some(immune) = IMMUNE.captures(property) {
                        for immune in immune[1].split(", ") {
                            immunes.insert(immune.parse::<Property>()?);
                        }
                    }
                }
            }
            Ok(Group {
                units,
                hp,
                weaks,
                immunes,
                atk,
                atk_type,
                initiative,
                ep: units * atk,

                target: None,
                targeted: None,
            })
        } else {
            Err(AoCError::DirtyInput)
        }
    }
}

impl Clone for Group {
    fn clone(&self) -> Self {
        Self {
            units: self.units,
            hp: self.hp,
            weaks: self.weaks.union(&HashSet::new()).cloned().collect(),
            immunes: self.immunes.union(&HashSet::new()).cloned().collect(),
            atk: self.atk,
            atk_type: self.atk_type,
            initiative: self.initiative,
            ep: self.ep,
            target: self.target,
            targeted: self.targeted,
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("units", &self.units)
            .field("hp", &self.hp)
            .field("weaks", &self.weaks)
            .field("immunes", &self.immunes)
            .field("atk", &self.atk)
            .field("atk_type", &self.atk_type)
            .field("initiative", &self.initiative)
            .field("ep", &self.ep)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Property {
    Fire,
    Cold,
    Radiation,
    Slashing,
    Bludgeoning,
}

impl FromStr for Property {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let res = match s {
            "fire" => Property::Fire,
            "cold" => Property::Cold,
            "radiation" => Property::Radiation,
            "slashing" => Property::Slashing,
            "bludgeoning" => Property::Bludgeoning,
            _ => return Err(AoCError::DirtyInput),
        };
        Ok(res)
    }
}

#[derive(PartialEq)]
enum Winner {
    ImmuneSystem,
    Infection,
    Deadlock,
}

impl From<usize> for Winner {
    fn from(idx: usize) -> Self {
        match idx {
            0 => Winner::ImmuneSystem,
            1 => Winner::Infection,
            _ => unreachable!(),
        }
    }
}
