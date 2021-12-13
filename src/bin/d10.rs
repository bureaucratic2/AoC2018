use std::{
    cell::RefCell,
    env::current_dir,
    fmt::{self, Display},
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
    str::{self, FromStr},
};

use aoc2018::{load, AoCError, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"position=<\s*([-]?\d+), \s*([-]?\d+)> velocity=<\s*([-]?\d+), \s*([-]?\d+)>")
            .unwrap();
}

fn main() -> Result<()> {
    let s = load(10);
    let mut points = Points::new();
    for line in s.lines() {
        points.push(line.parse()?);
    }

    let mut queue = StrangeQueue::new(Rc::new(RefCell::new(points.clone())));
    let mut count = 0;
    loop {
        if queue.push(points.volume()) {
            count = 0;
        } else {
            count += 1;
        }
        // points diverge, message has appeared.
        if count == 10 {
            break;
        }
        points.step();
    }

    let path = current_dir().unwrap().join("snapshot");
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);

    writer.write_fmt(format_args!("{}", queue))?;
    writer.flush()?;

    println!(
        "part1: read snapshot at {:?} and find answer 😊",
        path.as_path()
    );
    println!(
        "part2: read snapshot at {:?} and find answer 😊",
        path.as_path()
    );

    Ok(())
}

// do not store Points snapshot directly
// just store time and replay those move again
// time to space 😉
/// StrangeQueue is used to stored 10 snapshots with smallest area.
///
/// Maybe use a BinaryHeap is better?
struct StrangeQueue {
    inner: Vec<(i64, usize)>,
    min: i64,
    max: i64,

    display_replica: Rc<RefCell<Points>>,
}

impl StrangeQueue {
    fn new(replica: Rc<RefCell<Points>>) -> Self {
        Self {
            inner: vec![],
            min: i64::MAX,
            max: i64::MIN,

            display_replica: replica,
        }
    }

    /// Return false means input's area is larger than all snapshots in queue.
    fn push(&mut self, volume: (i64, usize)) -> bool {
        if self.inner.len() < 10 {
            self.inner.push(volume);
            self.min = self.min.min(volume.0);
            self.max = self.max.max(volume.0);
            true
        } else if volume.0 < self.max {
            self.inner.sort_by(|a, b| a.0.cmp(&b.0));

            self.inner.pop();
            self.max = self.inner.last().unwrap().0;

            self.inner.push(volume);
            self.min = self.min.min(volume.0);
            true
        } else {
            false
        }
    }

    fn sort_by_time(&self) -> Vec<usize> {
        let mut inner = self.inner.clone();
        inner.sort_by(|a, b| a.1.cmp(&b.1));
        inner.into_iter().map(|n| n.1).collect()
    }
}

impl Display for StrangeQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // now we know all 10 times, sort and replay them in files
        let times = self.sort_by_time();
        let mut replica = self.display_replica.borrow_mut();
        let mut prev_time = 0;

        for time in times {
            let iteration = time - prev_time;
            prev_time = time;
            for _ in 0..iteration {
                replica.step();
            }
            let (min_x, max_x, min_y, max_y) = replica.range();
            let x_ran = (max_x - min_x) as usize + 1;
            let y_ran = (max_y - min_y) as usize + 1;
            let mut v = vec![vec!['.'; x_ran]; y_ran];
            for point in replica.points.iter() {
                v[(point.position.1 - min_y) as usize][(point.position.0 - min_x) as usize] = '#';
            }

            writeln!(f, "snapshot at {}s", time)?;
            for row in v.iter() {
                writeln!(f, "{}", row.iter().collect::<String>())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Points {
    points: Vec<Point>,
    time: usize,
}

impl Points {
    fn new() -> Self {
        Self {
            points: vec![],
            time: 0,
        }
    }

    fn push(&mut self, point: Point) {
        self.points.push(point);
    }

    fn step(&mut self) {
        for point in self.points.iter_mut() {
            point.position.0 += point.velocity.0;
            point.position.1 += point.velocity.1;
        }
        self.time += 1;
    }

    fn volume(&self) -> (i64, usize) {
        let (min_x, max_x, min_y, max_y) = self.range();

        let volume = ((max_x - min_x) as i64) * ((max_y - min_y) as i64);

        (volume, self.time)
    }

    fn range(&self) -> (i32, i32, i32, i32) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for point in &self.points {
            min_x = min_x.min(point.position.0);
            max_x = max_x.max(point.position.0);

            min_y = min_y.min(point.position.1);
            max_y = max_y.max(point.position.1);
        }

        (min_x, max_x, min_y, max_y)
    }
}

#[derive(Debug, Clone)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Point {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            position: (caps[1].parse()?, caps[2].parse()?),
            velocity: (caps[3].parse()?, caps[4].parse()?),
        })
    }
}
