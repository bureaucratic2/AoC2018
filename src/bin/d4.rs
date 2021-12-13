use aoc2018::{load, AoCError, Result};
use chrono::{NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    static ref RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
}

fn main() -> Result<()> {
    let s = load(4);

    let mut logs = s.lines().collect::<Vec<_>>();
    // sort by chronological order
    logs.sort_unstable();

    let mut guards = HashMap::new();
    let mut latest = 0;
    let mut prev_sleep = 0;

    for log in logs {
        let log = log.parse::<Log>()?;
        match log.action {
            Action::Online(id) => {
                latest = id;
                prev_sleep = 0;
            }
            Action::Wake => {
                let entry = guards.entry(latest).or_insert_with(SleepMap::default);
                entry.total += log.time.minute() - prev_sleep;
                entry.range_map(prev_sleep, log.time.minute());
            }
            Action::Sleep => prev_sleep = log.time.minute(),
        }
    }

    part1(&guards);
    part2(&guards);
    Ok(())
}

fn part1(guards: &HashMap<u64, SleepMap>) {
    let id = guards
        .iter()
        .max_by(|x, y| x.1.total.cmp(&y.1.total))
        .map(|x| x.0)
        .unwrap();
    let sleep_map = guards.get(id).unwrap();
    let index_of_max_sleep = sleep_map
        .map
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .map(|x| x.0)
        .unwrap();

    println!("part1: {}", index_of_max_sleep as u64 * id);
}

fn part2(guards: &HashMap<u64, SleepMap>) {
    let res = guards
        .iter()
        .map(|(k, v)| {
            (
                k,
                v.map
                    .iter()
                    .enumerate()
                    .max_by(|x, y| x.1.cmp(y.1))
                    .unwrap(),
            )
        })
        .max_by(|x, y| x.1 .1.cmp(y.1 .1))
        .map(|x| x.1 .0 as u64 * x.0)
        .unwrap();

    println!("part2: {}", res);
}

#[derive(Debug)]
struct Log {
    time: NaiveDateTime,
    action: Action,
}

#[derive(Debug)]
enum Action {
    Online(u64),
    Wake,
    Sleep,
}

impl FromStr for Log {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let dt = &s[1..17];
        let time = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M")?;
        let action = match &s[19..] {
            "falls asleep" => Action::Sleep,
            "wakes up" => Action::Wake,
            s => {
                let caps = RE.captures(s).unwrap();
                Action::Online(caps[1].parse()?)
            }
        };
        Ok(Log { time, action })
    }
}
struct SleepMap {
    total: u32,
    map: Vec<u8>,
}

impl Default for SleepMap {
    fn default() -> Self {
        Self {
            total: 0,
            map: vec![0; 60],
        }
    }
}

impl SleepMap {
    fn range_map(&mut self, start: u32, end: u32) {
        for i in start..end {
            self.map[i as usize] += 1;
        }
    }
}
