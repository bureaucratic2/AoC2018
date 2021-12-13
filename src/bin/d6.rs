#![feature(int_abs_diff)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc2018::load;

fn main() {
    let s = load(6);
    let points = s
        .lines()
        .map(|s| {
            let mut coordinate = s.split(", ").map(|digit| digit.parse::<usize>().unwrap());
            let x = coordinate.next().unwrap();
            let y = coordinate.next().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let min_x = points.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = points.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let points = points
        .into_iter()
        .map(|point| (point.0 - min_x, point.1 - min_y))
        .collect::<Vec<_>>();

    let mut view = vec![vec![0; max_x - min_x]; max_y - min_y];

    part1(&mut view, &points);

    drop(view);

    // extend to ensure that it contains all possible points.
    let stretch = 10000 / points.len();
    let x_len = max_x - min_x + 2 * stretch;
    let y_len = max_y - min_y + 2 * stretch;
    let points = points
        .into_iter()
        .map(|point| (point.0 + stretch, point.1 + stretch))
        .collect::<Vec<_>>();

    part2(&points, x_len, y_len);
}

fn part1(view: &mut Vec<Vec<usize>>, points: &[(usize, usize)]) {
    let limit = points.len();
    let phantom = limit + 1;

    for (j, row) in view.iter_mut().enumerate() {
        for (i, index) in row.iter_mut().enumerate() {
            let mut duplicate = false;
            let mut closet = (usize::MAX, phantom);
            for (id, point) in points.iter().enumerate() {
                let dist = distance(point, &(i, j));
                match dist.cmp(&closet.0) {
                    Ordering::Less => {
                        closet.0 = dist;
                        closet.1 = id;
                        duplicate = false;
                    }
                    Ordering::Equal => duplicate = true,
                    Ordering::Greater => {}
                }
            }
            *index = if duplicate { phantom } else { closet.1 };
        }
    }

    // if a point's area reach edge, it has infinite area.
    let mut infinite = HashSet::new();
    for id in view.first().unwrap() {
        infinite.insert(*id);
    }
    for id in view.last().unwrap() {
        infinite.insert(*id);
    }
    for list in view.iter() {
        infinite.insert(*list.first().unwrap());
        infinite.insert(*list.last().unwrap());
    }

    // mark those infinite points' area.
    for row in view.iter_mut() {
        for index in row.iter_mut() {
            if infinite.contains(index) {
                *index = limit;
            }
        }
    }

    let mut count = HashMap::new();

    for row in view.iter_mut() {
        for index in row.iter_mut() {
            if index != &limit && index != &phantom {
                *count.entry(*index).or_insert(0) += 1;
            }
        }
    }

    println!("part1: {}", count.values().max().unwrap());
}

fn part2(points: &[(usize, usize)], x_len: usize, y_len: usize) {
    let mut count = 0;
    for x in 0..x_len {
        for y in 0..y_len {
            let mut dist = 0;
            for point in points {
                dist += distance(point, &(x, y));
                if dist > 10000 {
                    break;
                }
            }
            if dist < 10000 {
                count += 1;
            }
        }
    }
    println!("part2: {}", count);
}

fn distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}
