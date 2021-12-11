type GridMatrix = [[i64; GRID_SIDE + 2]; GRID_SIDE + 2];

const GRID_SIDE: usize = 10;

// (x, y)
const OFFSETS: [(i64, i64); 8] = [
    (-1, -1), // left upper
    (0, -1),  // upper
    (1, -1),  // right upper
    (-1, 0),  // left
    (1, 0),   // right
    (-1, 1),  // left lower
    (0, 1),   // lower
    (1, 1),   // right lower
];

#[inline(always)]
unsafe fn try_flashing(grid: &mut GridMatrix, x: i64, y: i64) {
    let point = grid
        .get_unchecked_mut(y as usize)
        .get_unchecked_mut(x as usize);

    *point += 1;

    if *point == b':' as i64 {
        for (off_x, off_y) in OFFSETS {
            try_flashing(grid, x + off_x, y + off_y);
        }
    }
}

#[inline(always)]
unsafe fn reset_flashed_counting(grid: &mut GridMatrix) -> i64 {
    let mut flashed = 0;

    for i in 1..GRID_SIDE + 1 {
        for j in 1..GRID_SIDE + 1 {
            let point = grid.get_unchecked_mut(i).get_unchecked_mut(j);

            if *point > b'9' as i64 {
                *point = b'0' as i64;

                flashed += 1;
            }
        }
    }

    flashed
}

pub fn run1(input: &[u8]) -> i64 {
    let mut grid = [[i64::MIN; GRID_SIDE + 2]; GRID_SIDE + 2];

    let mut answer = 0;

    unsafe {
        for y in 0..GRID_SIDE {
            for x in 0..GRID_SIDE {
                grid[y + 1][x + 1] = input[y * GRID_SIDE + x + y] as i64;
            }
        }

        for _ in 0..100 {
            for y in 1..GRID_SIDE + 1 {
                for x in 1..GRID_SIDE + 1 {
                    try_flashing(&mut grid, x as i64, y as i64);
                }
            }

            answer += reset_flashed_counting(&mut grid);
        }
    }

    answer
}

pub fn run2(input: &[u8]) -> i64 {
    let mut grid = [[i64::MIN; GRID_SIDE + 2]; GRID_SIDE + 2];

    let mut day = 1;

    unsafe {
        for y in 0..GRID_SIDE {
            for x in 0..GRID_SIDE {
                grid[y + 1][x + 1] = input[y * GRID_SIDE + x + y] as i64;
            }
        }

        loop {
            for y in 1..GRID_SIDE + 1 {
                for x in 1..GRID_SIDE + 1 {
                    try_flashing(&mut grid, x as i64, y as i64);
                }
            }

            if reset_flashed_counting(&mut grid) == (GRID_SIDE * GRID_SIDE) as i64 {
                return day;
            }

            day += 1;
        }
    }
}
