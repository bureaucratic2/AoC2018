#![feature(linked_list_cursors)]

use aoc2018::{load, Result};
use regex::Regex;
use std::collections::{LinkedList, VecDeque};

fn main() -> Result<()> {
    let s = load(9);
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points")?;
    let input = s.lines().next().unwrap();
    let caps = re.captures(input).unwrap();
    let (players, points) = (caps[1].parse::<usize>()?, caps[2].parse::<u64>()?);

    part1(players, points);
    part2(players, 100 * points);
    Ok(())
}

fn part1(players: usize, points: u64) {
    println!("part1: {}", vec(players, points));
}

fn part2(players: usize, points: u64) {
    println!("part2: {}", list(players, points));
}

/// Use VecDeque to simulate.
fn vec(players: usize, points: u64) -> u64 {
    if points < 23 {
        return 0;
    }

    let mut scores = vec![0; players];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    queue.push_back(1);
    let mut pointer = 1;
    let mut player = 1;

    for i in 2..points + 1 {
        player = (player + 1) % players;
        if i % 23 != 0 {
            pointer = (pointer + 1) % queue.len();
            pointer += 1;
            if pointer == 1 {
                let zero = queue.pop_front().unwrap();
                queue.push_front(i);
                queue.push_front(zero);
            } else {
                queue.insert(pointer, i);
            }
        } else {
            // care about overflow
            pointer = (pointer + queue.len() - 7) % queue.len();

            let score = queue.remove(pointer).unwrap();
            scores[player] += score + i;
            pointer %= queue.len();
        }
    }

    *scores.iter().max().unwrap()
}

/// Use LinkedList to simulate.
fn list(players: usize, points: u64) -> u64 {
    if points < 23 {
        return 0;
    }

    let mut scores = vec![0; players];
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    let mut pointer = list.cursor_back_mut();
    let mut player = 1;

    for i in 2..points + 1 {
        player = (player + 1) % players;
        if i % 23 != 0 {
            pointer.move_next();
            if pointer.current().is_none() {
                pointer.move_next();
            }
            pointer.insert_after(i);
            pointer.move_next();
        } else {
            for _ in 0..7 {
                pointer.move_prev();
                if pointer.current().is_none() {
                    pointer.move_prev();
                }
            }
            let score = pointer.remove_current().unwrap();
            if pointer.current().is_none() {
                pointer.move_next();
            }
            scores[player] += i + score;
        }
    }

    *scores.iter().max().unwrap()
}

#[test]
fn linked_list() {
    let mut list = LinkedList::from([1, 2, 3]);
    let mut cursor = list.cursor_front_mut();
    for i in 1..4 {
        assert_eq!(cursor.current().unwrap(), &i);
        cursor.move_next();
    }
    assert_eq!(cursor.current(), None);
    cursor.move_next();
    for i in 1..4 {
        assert_eq!(cursor.current().unwrap(), &i);
        cursor.move_next();
    }
    cursor.insert_before(4);
    cursor.move_next();
    println!("{:?}", cursor.current());
    println!("{:?}", list);
}
