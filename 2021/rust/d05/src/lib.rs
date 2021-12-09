#![feature(int_abs_diff)]

use std::cmp::{max, min};

const VECTOR_COUNT: usize = 500;

const GRID_SIDE: usize = 1000;

#[allow(dead_code)]
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

#[inline(always)]
fn count_dangerous(grid: &[i64; GRID_SIDE * GRID_SIDE]) -> usize {
    grid.iter().filter(|&v| *v > 1).count()
}

#[inline(always)]
unsafe fn next_number(input: &[u8], pointer: &mut usize, stop_at: u8) -> usize {
    let mut number = 0;

    loop {
        let byte = *input.get_unchecked(*pointer);

        *pointer += 1;

        if byte == stop_at {
            return number;
        }

        number = number * 10 + (byte - b'0') as usize;
    }
}

pub fn run1(input: &[u8]) -> i64 {
    let mut pointer = 0;

    let mut grid = [0; GRID_SIDE * GRID_SIDE];

    unsafe {
        for _ in 0..VECTOR_COUNT {
            let x1 = next_number(input, &mut pointer, b',');
            let y1 = next_number(input, &mut pointer, b' ');

            // skip "-> "
            pointer += 3;

            let x2 = next_number(input, &mut pointer, b',');
            let y2 = next_number(input, &mut pointer, b'\n');

            if y1 == y2 {
                let start_x = min(x1, x2);

                for i in start_x..=start_x + x1.abs_diff(x2) {
                    *grid.get_unchecked_mut(y1 * GRID_SIDE + i) += 1;
                }
            } else if x1 == x2 {
                let start_y = min(y1, y2);

                for i in start_y..=start_y + y1.abs_diff(y2) {
                    *grid.get_unchecked_mut(i * GRID_SIDE + x1) += 1;
                }
            }
        }
    }

    // print_grid(&grid);

    count_dangerous(&grid) as i64
}

pub fn run2(input: &[u8]) -> i64 {
    let mut pointer = 0;

    let mut grid = [0; GRID_SIDE * GRID_SIDE];

    unsafe {
        for _ in 0..VECTOR_COUNT {
            let x1 = next_number(input, &mut pointer, b',');
            let y1 = next_number(input, &mut pointer, b' ');

            // skip "-> "
            pointer += 3;

            let x2 = next_number(input, &mut pointer, b',');
            let y2 = next_number(input, &mut pointer, b'\n');
            if y1 == y2 {
                let start_x = min(x1, x2);

                for i in start_x..=start_x + x1.abs_diff(x2) {
                    *grid.get_unchecked_mut(y1 * GRID_SIDE + i) += 1;
                }
            } else if x1 == x2 {
                let start_y = min(y1, y2);

                for i in start_y..=start_y + y1.abs_diff(y2) {
                    *grid.get_unchecked_mut(i * GRID_SIDE + x1) += 1;
                }
            } else {
                let dist_x = x2 as i64 - x1 as i64;
                let dist_y = y2 as i64 - y1 as i64;

                let start_x = min(x1, x2);

                if dist_x - dist_y == 0 {
                    let start_y = min(y1, y2);
                    for i in 0..=dist_x.abs() as usize {
                        *grid.get_unchecked_mut((start_y + i) * GRID_SIDE + start_x + i) += 1;
                    }
                } else {
                    let start_y = max(y1, y2);
                    for i in 0..=dist_x.abs() as usize {
                        *grid.get_unchecked_mut((start_y - i) * GRID_SIDE + start_x + i) += 1;
                    }
                }
            }
        }
    }

    // print_grid(&grid);

    count_dangerous(&grid) as i64
}
