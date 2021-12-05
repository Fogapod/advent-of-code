#![feature(int_abs_diff)]

use std::cmp::{max, min};
use std::fs;

const GRID_SIDE: usize = 1000;

fn print_grid(grid: &[i64; GRID_SIDE * GRID_SIDE]) {
    for i in 0..GRID_SIDE {
        for j in 0..GRID_SIDE {
            print!(
                "{}",
                match grid[i * GRID_SIDE + j] {
                    0 => '.',
                    1 => '1',
                    2 => '2',
                    _ => '!',
                }
            )
        }

        println!();
    }
}

fn count_dangerous(grid: &[i64; GRID_SIDE * GRID_SIDE]) -> usize {
    grid.iter().filter(|&v| *v > 1).count()
}

pub fn run1(input: &str) -> i64 {
    let vectors = input.lines().map(|line| {
        let (x1y1, x2y2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = x1y1.split_once(',').unwrap();
        let (x2, y2) = x2y2.split_once(',').unwrap();

        (
            x1.parse::<usize>().unwrap(),
            y1.parse::<usize>().unwrap(),
            x2.parse::<usize>().unwrap(),
            y2.parse::<usize>().unwrap(),
        )
    });

    let mut grid = [0; GRID_SIDE * GRID_SIDE];

    for (x1, y1, x2, y2) in vectors {
        if y1 == y2 {
            let start_x = min(x1, x2);

            for i in start_x..=start_x + x1.abs_diff(x2) {
                grid[y1 * GRID_SIDE + i] += 1;
            }
        } else if x1 == x2 {
            let start_y = min(y1, y2);

            for i in start_y..=start_y + y1.abs_diff(y2) {
                grid[i * GRID_SIDE + x1] += 1;
            }
        }
    }

    // print_grid(&grid);

    count_dangerous(&grid) as i64
}

pub fn run2(input: &str) -> i64 {
    let vectors = input.lines().map(|line| {
        let (x1y1, x2y2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = x1y1.split_once(',').unwrap();
        let (x2, y2) = x2y2.split_once(',').unwrap();

        (
            x1.parse::<usize>().unwrap(),
            y1.parse::<usize>().unwrap(),
            x2.parse::<usize>().unwrap(),
            y2.parse::<usize>().unwrap(),
        )
    });

    let mut grid = [0; GRID_SIDE * GRID_SIDE];

    for (x1, y1, x2, y2) in vectors {
        if y1 == y2 {
            let start_x = min(x1, x2);

            for i in start_x..=start_x + x1.abs_diff(x2) {
                grid[y1 * GRID_SIDE + i] += 1;
            }
        } else if x1 == x2 {
            let start_y = min(y1, y2);

            for i in start_y..=start_y + y1.abs_diff(y2) {
                grid[i * GRID_SIDE + x1] += 1;
            }
        } else {
            let dist_x = x2 as i64 - x1 as i64;
            let dist_y = y2 as i64 - y1 as i64;

            let start_x = min(x1, x2);

            if dist_x - dist_y == 0 {
                let start_y = min(y1, y2);
                for i in 0..=dist_x.abs() as usize {
                    grid[(start_y + i) * GRID_SIDE + start_x + i] += 1;
                }
            } else {
                let start_y = max(y1, y2);
                for i in 0..=dist_x.abs() as usize {
                    grid[(start_y - i) * GRID_SIDE + start_x + i] += 1;
                }
            }
        }
    }

    // print_grid(&grid);

    count_dangerous(&grid) as i64
}
fn main() {
    let input = &fs::read_to_string("input").unwrap();

    println!("{}", run1(input));
    println!("{}", run2(input));
}
