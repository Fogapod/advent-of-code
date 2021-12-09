#![feature(unchecked_math)]

// 5 and 10 for example, 100 and 100 for rull
const GRID_HEIGHT: usize = 100;
const GRID_WIDTH: usize = 100;

pub fn run1(input: &[u8]) -> i64 {
    let mut grid = [[b'9'; GRID_WIDTH + 2]; GRID_HEIGHT + 2];

    let mut answer: i64 = 0;

    unsafe {
        for y in 1..GRID_HEIGHT + 1 {
            let offset = y
                .unchecked_sub(1)
                .unchecked_mul(GRID_WIDTH.unchecked_add(1));

            grid.get_unchecked_mut(y)
                .get_unchecked_mut(1..GRID_WIDTH + 1)
                .copy_from_slice(&input[offset..offset + GRID_WIDTH]);
        }

        for y in 1..GRID_HEIGHT + 1 {
            let mut x = 1;

            loop {
                let point = *grid.get_unchecked(y).get_unchecked(x);

                if *grid.get_unchecked(y.unchecked_sub(1)).get_unchecked(x) <= point
                    || *grid.get_unchecked(y).get_unchecked(x.unchecked_sub(1)) <= point
                    || *grid.get_unchecked(y.unchecked_add(1)).get_unchecked(x) <= point
                    || *grid.get_unchecked(y).get_unchecked(x.unchecked_add(1)) <= point
                {
                    x += 1;

                    // putting this check above previous line hurts perfomance
                    if x > GRID_WIDTH {
                        break;
                    }

                    continue;
                } else {
                    // we just checked point on our right, skip it
                    x += 2;
                }

                answer = answer.unchecked_add((point - b'/') as i64);

                if x > GRID_WIDTH {
                    break;
                }
            }
        }
    }

    answer
}

unsafe fn flood(grid: &mut [[u8; GRID_WIDTH + 2]; GRID_HEIGHT + 2], x: usize, y: usize) -> i64 {
    let point = grid.get_unchecked_mut(y).get_unchecked_mut(x);

    if *point == b'9' {
        return 0;
    }

    *point = b'9';

    1 + flood(grid, x - 1, y)
        + flood(grid, x, y - 1)
        + flood(grid, x + 1, y)
        + flood(grid, x, y + 1)
}

pub fn run2(input: &[u8]) -> i64 {
    let mut grid = [[b'9'; GRID_WIDTH + 2]; GRID_HEIGHT + 2];

    let mut three_largest_floods = [0i64; 3];

    unsafe {
        for y in 1..GRID_HEIGHT + 1 {
            let offset = y
                .unchecked_sub(1)
                .unchecked_mul(GRID_WIDTH.unchecked_add(1));

            grid.get_unchecked_mut(y)
                .get_unchecked_mut(1..GRID_WIDTH + 1)
                .copy_from_slice(&input[offset..offset + GRID_WIDTH]);
        }

        for y in 1..GRID_HEIGHT + 1 {
            for x in 1..GRID_WIDTH + 1 {
                let point = *grid.get_unchecked(y).get_unchecked(x);

                if point == b'9' {
                    continue;
                }

                if *grid.get_unchecked(y.unchecked_sub(1)).get_unchecked(x) <= point {
                    continue;
                }
                if *grid.get_unchecked(y).get_unchecked(x.unchecked_sub(1)) <= point {
                    continue;
                }
                if *grid.get_unchecked(y.unchecked_add(1)).get_unchecked(x) <= point {
                    continue;
                }
                if *grid.get_unchecked(y).get_unchecked(x.unchecked_add(1)) <= point {
                    continue;
                }

                let flooded = flood(&mut grid, x, y);

                if flooded >= three_largest_floods[0] {
                    three_largest_floods[2] = three_largest_floods[1];
                    three_largest_floods[1] = three_largest_floods[0];
                    three_largest_floods[0] = flooded;
                } else if flooded >= three_largest_floods[1] {
                    three_largest_floods[2] = three_largest_floods[1];
                    three_largest_floods[1] = flooded;
                } else if flooded >= three_largest_floods[2] {
                    three_largest_floods[2] = flooded;
                }

                // no perfomance difference
                //
                // if flooded >= *three_largest_floods.get_unchecked(0) {
                //     *three_largest_floods.get_unchecked_mut(2) =
                //         *three_largest_floods.get_unchecked(1);
                //     *three_largest_floods.get_unchecked_mut(1) =
                //         *three_largest_floods.get_unchecked(0);
                //     *three_largest_floods.get_unchecked_mut(0) = flooded;
                // } else if flooded >= *three_largest_floods.get_unchecked(1) {
                //     *three_largest_floods.get_unchecked_mut(2) =
                //         *three_largest_floods.get_unchecked(1);
                //     *three_largest_floods.get_unchecked_mut(1) = flooded;
                // } else if flooded >= *three_largest_floods.get_unchecked(2) {
                //     *three_largest_floods.get_unchecked_mut(2) = flooded;
                // }
            }
        }
    }

    three_largest_floods[0] * three_largest_floods[1] * three_largest_floods[2]
}
