use aoc2018::{load, AoCError};
use std::{cmp, collections::HashSet};

const TURN: [Turn; 3] = [Turn::Left, Turn::Straight, Turn::Right];

type Tracks = Vec<Vec<char>>;

fn main() -> Result<(), AoCError> {
    let s = load(13);
    let mut tracks: Tracks = Vec::new();
    for line in s.lines() {
        tracks.push(line.chars().collect::<Vec<_>>());
    }
    let mut origin = tracks.clone();
    let mut carts = Carts::new();
    extract_carts_from_tracks(&mut origin, &mut carts);
    carts.tracks.push(origin);
    carts.tracks.push(tracks);

    part1(carts.clone());
    part2(carts);

    Ok(())
}

fn part1(mut carts: Carts) {
    loop {
        if let Some((x, y)) = carts.tick() {
            println!("part1: {} {}", x, y);
            break;
        }
    }
}

fn part2(mut carts: Carts) {
    let (x, y) = carts.tick_with_collisions_avoid();
    println!("part2: {} {}", x, y);
}

#[derive(Debug, Clone)]
struct Carts {
    carts: Vec<Cart>,
    tracks: Vec<Tracks>,
    labels: HashSet<char>,
}

impl Carts {
    fn new() -> Self {
        let mut labels = HashSet::with_capacity(4);
        labels.insert('<');
        labels.insert('>');
        labels.insert('^');
        labels.insert('v');
        Self {
            carts: vec![],
            tracks: vec![],
            labels,
        }
    }

    fn push(&mut self, cart: Cart) {
        self.carts.push(cart);
    }

    fn sort_carts(&mut self) {
        self.carts
            .sort_unstable_by(|a, b| match a.pos.1.cmp(&b.pos.1) {
                cmp::Ordering::Equal => a.pos.0.cmp(&b.pos.0),
                _ => a.pos.1.cmp(&b.pos.1),
            });
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        self.sort_carts();

        for cart in self.carts.iter_mut() {
            if let Some((x, y)) = cart.step(&mut self.tracks, &self.labels) {
                return Some((x, y));
            }
        }

        None
    }

    fn tick_with_collisions_avoid(&mut self) -> (usize, usize) {
        let mut index = 0;
        loop {
            let cart = &mut self.carts[index];
            if let Some((x, y)) = cart.step(&mut self.tracks, &self.labels) {
                let mut iter = self
                    .carts
                    .iter()
                    .enumerate()
                    .filter(|&(_index, cart)| cart.pos.0 == x && cart.pos.1 == y)
                    .map(|(index, _cart)| index);

                let (prev, next) = (iter.next().unwrap(), iter.next().unwrap());
                self.carts.remove(next);
                self.carts.remove(prev);
                if next == index {
                    index -= 1;
                }
            } else {
                index += 1;
            }
            // a tick ends
            if index == self.carts.len() {
                self.sort_carts();
                index = 0;
                if self.carts.len() == 1 {
                    return self.carts[0].pos;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Cart {
    pos: (usize, usize),
    front: Direction,
    next_turn: usize,
}

impl Cart {
    fn new(pos: (usize, usize), front: Direction) -> Self {
        Self {
            pos,
            front,
            next_turn: 0,
        }
    }

    fn step(&mut self, tracks: &mut Vec<Tracks>, labels: &HashSet<char>) -> Option<(usize, usize)> {
        let (old_x, old_y) = self.pos;

        // move cart
        match self.front {
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Right => self.pos.0 += 1,
        }

        // check collisions
        let (x, y) = self.pos;
        if labels.contains(&tracks[1][y][x]) {
            // recover tracks
            tracks[1][old_y][old_x] = tracks[0][old_y][old_x];
            tracks[1][y][x] = tracks[0][y][x];
            return Some((x, y));
        }

        // upadte direction
        let track = tracks[0][y][x];
        match track {
            '\\' => match self.front {
                Direction::Up => self.front = Direction::Left,
                Direction::Down => self.front = Direction::Right,
                Direction::Left => self.front = Direction::Up,
                Direction::Right => self.front = Direction::Down,
            },

            '/' => match self.front {
                Direction::Up => self.front = Direction::Right,
                Direction::Down => self.front = Direction::Left,
                Direction::Left => self.front = Direction::Down,
                Direction::Right => self.front = Direction::Up,
            },
            '+' => {
                match self.front {
                    Direction::Up => {
                        self.front = match TURN[self.next_turn] {
                            Turn::Left => Direction::Left,
                            Turn::Straight => Direction::Up,
                            Turn::Right => Direction::Right,
                        }
                    }
                    Direction::Down => {
                        self.front = match TURN[self.next_turn] {
                            Turn::Left => Direction::Right,
                            Turn::Straight => Direction::Down,
                            Turn::Right => Direction::Left,
                        }
                    }
                    Direction::Left => {
                        self.front = match TURN[self.next_turn] {
                            Turn::Left => Direction::Down,
                            Turn::Straight => Direction::Left,
                            Turn::Right => Direction::Up,
                        }
                    }
                    Direction::Right => {
                        self.front = match TURN[self.next_turn] {
                            Turn::Left => Direction::Up,
                            Turn::Straight => Direction::Right,
                            Turn::Right => Direction::Down,
                        }
                    }
                }
                self.next_turn += 1;
                self.next_turn %= TURN.len();
            }
            _ => {}
        }

        // update tracks
        let new_location = &mut tracks[1][y][x];
        match self.front {
            Direction::Up => *new_location = '^',
            Direction::Down => *new_location = 'v',
            Direction::Left => *new_location = '<',
            Direction::Right => *new_location = '>',
        }

        // recover tracks
        tracks[1][old_y][old_x] = tracks[0][old_y][old_x];

        None
    }
}

fn extract_carts_from_tracks(tracks: &mut Tracks, carts: &mut Carts) {
    for (y, track) in tracks.iter_mut().enumerate() {
        for (x, ch) in track.iter_mut().enumerate() {
            match *ch {
                '<' => {
                    carts.push(Cart::new((x, y), Direction::Left));
                    *ch = '-';
                }
                '>' => {
                    carts.push(Cart::new((x, y), Direction::Right));
                    *ch = '-';
                }
                '^' => {
                    carts.push(Cart::new((x, y), Direction::Up));
                    *ch = '|';
                }
                'v' => {
                    carts.push(Cart::new((x, y), Direction::Down));
                    *ch = '|';
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Turn {
    Left,
    Straight,
    Right,
}
