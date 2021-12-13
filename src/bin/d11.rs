use aoc2018::{load, AoCError};

const LEN: usize = 300;

fn main() -> Result<(), AoCError> {
    let s = load(11);
    let serial = s.parse::<i64>()?;

    let grid = Grid::new(serial);
    part1(&grid);
    part2(&grid);

    Ok(())
}

fn part1(grid: &Grid) {
    let res = grid.max(3);
    println!("part1: {},{}", res.0, res.1);
}

/// Summed Area Table is used to reduce complexity from O(n^5) to O(N^3)
fn part2(grid: &Grid) {
    let res = grid.max(0);
    println!("part2: {},{},{}", res.0, res.1, res.2);
}

#[derive(Debug)]
struct Grid {
    cells: [[i64; LEN]; LEN],
    sum_table: [[i64; LEN]; LEN],
    serial: i64,
}

impl Grid {
    fn new(serial: i64) -> Self {
        let mut grid = Self {
            cells: [[0; LEN]; LEN],
            sum_table: [[0; LEN]; LEN],
            serial,
        };
        grid.power();
        grid.sum_table();
        grid
    }

    fn power(&mut self) {
        for (y, row) in self
            .cells
            .iter_mut()
            .enumerate()
            .map(|cell| (cell.0 as i64 + 1, cell.1))
        {
            for (x, cell) in row
                .iter_mut()
                .enumerate()
                .map(|cell| (cell.0 as i64 + 1, cell.1))
            {
                *cell = power(x, y, self.serial);
            }
        }
    }

    fn sum_table(&mut self) {
        let x_len = self.sum_table[0].len();
        let y_len = self.sum_table.len();

        self.sum_table[0][x_len - 1] = self.cells[0].iter().sum();
        for x in (0..self.sum_table[0].len() - 1).rev() {
            self.sum_table[0][x] = self.sum_table[0][x + 1] - self.cells[0][x + 1];
        }

        self.sum_table[y_len - 1][0] = self.cells.iter().map(|list| list[0]).sum();
        for y in (0..y_len - 1).rev() {
            self.sum_table[y][0] = self.sum_table[y + 1][0] - self.cells[y + 1][0];
        }

        for y in 1..y_len {
            for x in 1..x_len {
                self.sum_table[y][x] =
                    self.sum_table[y - 1][x] + self.sum_table[y][x - 1] + self.cells[y][x]
                        - self.sum_table[y - 1][x - 1];
            }
        }
    }

    // range zero means any range
    fn max(&self, range: usize) -> (usize, usize, usize) {
        let x_len = self.sum_table[0].len();
        let y_len = self.sum_table.len();
        if range > 0 {
            let mut max = i64::MIN;
            let mut coordinate = (0, 0);
            for y in 0..=y_len - range {
                for x in 0..=x_len - range {
                    if max < self.power_square(x, y, range) {
                        max = self.power_square(x, y, range);
                        coordinate.0 = x + 1;
                        coordinate.1 = y + 1;
                    }
                }
            }
            (coordinate.0, coordinate.1, range)
        } else {
            let mut max_range = 0;
            let mut max = i64::MIN;
            let mut coordinate = (0, 0);
            for range in 1..y_len {
                for y in 0..=y_len - range {
                    for x in 0..=x_len - range {
                        if max < self.power_square(x, y, range) {
                            max = self.power_square(x, y, range);
                            coordinate.0 = x + 1;
                            coordinate.1 = y + 1;
                            max_range = range;
                        }
                    }
                }
            }
            (coordinate.0, coordinate.1, max_range)
        }
    }

    fn power_square(&self, x: usize, y: usize, mut range: usize) -> i64 {
        if range == 1 {
            self.cells[y][x]
        } else {
            range -= 1;
            match (x, y) {
                (0, 0) => self.sum_table[y + range][x + range],
                (0, y) => self.sum_table[y + range][x + range] - self.sum_table[y - 1][x + range],
                (x, 0) => self.sum_table[y + range][x + range] - self.sum_table[y + range][x - 1],
                (x, y) => {
                    self.sum_table[y - 1][x - 1] + self.sum_table[y + range][x + range]
                        - self.sum_table[y - 1][x + range]
                        - self.sum_table[y + range][x - 1]
                }
            }
        }
    }
}

fn power(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = hundreds(power_level) - 5;
    power_level
}

#[inline]
fn hundreds(num: i64) -> i64 {
    num / 100 % 10
}
