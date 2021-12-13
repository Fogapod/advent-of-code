#![feature(unchecked_math)]

// assumption, values greater than this overflow stack size
const MAX_GRID_SIDE: usize = 1400;

const FOLD_COUNT: usize = 12;

// after first fold
const FINAL_GRID_HEIGHT: usize = 1000;
const FINAL_GRID_WIDTH: usize = 1000;

pub fn run1(input: &str) -> i64 {
    let mut grid = [[0; FINAL_GRID_WIDTH]; FINAL_GRID_HEIGHT];

    let mut pointer: usize = 0;

    let (points, folds) = input.split_once("\n\n").unwrap();

    let folds = folds.as_bytes();
    let points = points.as_bytes();

    unsafe {
        pointer = pointer.unchecked_add(11);
        let first_fold_axis = *folds.get_unchecked(pointer);
        pointer = pointer.unchecked_add(2);

        let byte = folds.get_unchecked(pointer);
        pointer = pointer.unchecked_add(1);

        let mut first_fold_index = byte.unchecked_sub(b'0') as usize;
        loop {
            let byte = *folds.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);

            if byte == b'\n' {
                break;
            }

            first_fold_index = first_fold_index
                .unchecked_mul(10)
                .unchecked_add(byte.unchecked_sub(b'0') as usize);
        }

        pointer = pointer.unchecked_add(13);

        let byte = folds.get_unchecked(pointer);
        pointer = pointer.unchecked_add(1);

        let mut second_fold_index = byte.unchecked_sub(b'0') as usize;
        loop {
            let byte = *folds.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);

            if byte == b'\n' {
                break;
            }

            second_fold_index = second_fold_index
                .unchecked_mul(10)
                .unchecked_add(byte.unchecked_sub(b'0') as usize);
        }

        let (final_width, final_height) = {
            if first_fold_axis == b'x' {
                (first_fold_index, second_fold_index * 2 + 1)
            } else {
                (second_fold_index * 2 + 1, first_fold_index)
            }
        };

        let mut pointer: usize = 0;

        loop {
            let byte = *points.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);

            if byte == b'\n' {
                // go to folds parsing
                break;
            }

            let mut coord_x = byte.unchecked_sub(b'0') as i64;

            loop {
                let byte = *points.get_unchecked(pointer);
                pointer = pointer.unchecked_add(1);

                if byte == b',' {
                    break;
                }

                coord_x = coord_x
                    .unchecked_mul(10)
                    .unchecked_add(byte.unchecked_sub(b'0') as i64);
            }

            let byte = *points.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);
            let mut coord_y = byte.unchecked_sub(b'0') as i64;

            loop {
                let byte = *points.get_unchecked(pointer);
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                coord_y = coord_y
                    .unchecked_mul(10)
                    .unchecked_add(byte.unchecked_sub(b'0') as i64);
            }

            if first_fold_axis == b'x' {
                *grid.get_unchecked_mut(coord_y as usize).get_unchecked_mut(
                    i64::abs(coord_x.unchecked_sub(final_width as i64)) as usize - 1,
                ) = 1;
            } else {
                *grid
                    .get_unchecked_mut(
                        i64::abs(coord_y.unchecked_sub(final_height as i64)) as usize - 1,
                    )
                    .get_unchecked_mut(coord_x as usize) = 1;
            }
        }

        // (0..final_height).for_each(|y| {
        //     for x in 0..final_width {
        //         if grid[y][x] > 0 {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // });
        // println!();

        let mut answer = 0;

        (0..final_height).for_each(|y| {
            for x in 0..final_width {
                if *grid.get_unchecked(y).get_unchecked(x) == 1 {
                    answer += 1;
                }
            }
        });

        answer
    }
}

pub fn run2(input: &[u8]) -> i64 {
    const FINAL_GRID_HEIGHT: usize = 6;
    const FINAL_GRID_WIDTH: usize = 5 * 8;

    let mut grid = [[0; MAX_GRID_SIDE]; MAX_GRID_SIDE];

    let mut pointer = 0;

    let mut size_x = 0;
    let mut size_y = 0;

    unsafe {
        loop {
            let byte = *input.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);

            if byte == b'\n' {
                // go to folds parsing
                break;
            }

            let mut coord_x = byte.unchecked_sub(b'0') as usize;

            loop {
                let byte = *input.get_unchecked(pointer);
                pointer = pointer.unchecked_add(1);

                if byte == b',' {
                    break;
                }

                coord_x = coord_x
                    .unchecked_mul(10)
                    .unchecked_add(byte.unchecked_sub(b'0') as usize);
            }

            let byte = *input.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);
            let mut coord_y = byte.unchecked_sub(b'0') as usize;

            loop {
                let byte = *input.get_unchecked(pointer);
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                coord_y = coord_y
                    .unchecked_mul(10)
                    .unchecked_add(byte.unchecked_sub(b'0') as usize);
            }

            if coord_x > size_x {
                size_x = coord_x;
            }

            if coord_y > size_y {
                size_y = coord_y;
            }

            *grid.get_unchecked_mut(coord_y).get_unchecked_mut(coord_x) = 1;
        }

        size_x += 1;
        size_y += 1;

        for _ in 0..FOLD_COUNT {
            pointer = pointer.unchecked_add(11);
            let fold_axis = input.get_unchecked(pointer);
            pointer = pointer.unchecked_add(2);

            let byte = input.get_unchecked(pointer);
            pointer = pointer.unchecked_add(1);
            let mut fold_index = byte.unchecked_sub(b'0') as usize;

            loop {
                let byte = *input.get_unchecked(pointer);
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                fold_index = fold_index
                    .unchecked_mul(10)
                    .unchecked_add(byte.unchecked_sub(b'0') as usize);
            }

            if *fold_axis == b'x' {
                (0..size_y).for_each(|y| {
                    for x in 0..fold_index {
                        *grid.get_unchecked_mut(y).get_unchecked_mut(x) += *grid
                            .get_unchecked(y)
                            .get_unchecked(size_x.unchecked_sub(x).unchecked_sub(1));
                    }
                });

                size_x -= fold_index + 1;
            } else {
                (0..fold_index).for_each(|y| {
                    for x in 0..size_x {
                        *grid.get_unchecked_mut(y).get_unchecked_mut(x) += *grid
                            .get_unchecked(size_y.unchecked_sub(y).unchecked_sub(1))
                            .get_unchecked(x);
                    }
                });

                size_y -= fold_index + 1;
            }
        }
    }

    // (0..FINAL_GRID_HEIGHT).for_each(|y| {
    //     for x in 0..FINAL_GRID_WIDTH {
    //         if grid[y][x] > 0 {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // });
    // println!();

    // for benchmarking competition
    let mut answer = 0;

    (0..FINAL_GRID_HEIGHT).for_each(|y| {
        for x in 0..FINAL_GRID_WIDTH {
            if grid[y][x] > 0 {
                answer += 1;
            }
        }
    });

    answer
}
