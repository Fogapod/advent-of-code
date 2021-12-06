use std::fs;

const BOARD_SIDE: usize = 5;
const BOARD_COUNT: usize = 100;

// 10 numbers (0 - 9) + 90 numbers (10 - 99) + 99 ','
const RAW_NUMBERS_LENGTH: usize = 10 + 90 * 2 + 99;

const RAW_BOARD_ROW_LENGTH: usize = BOARD_SIDE * 2 + BOARD_SIDE - 1 + 1;
const RAW_BOARD_LENGTH: usize = RAW_BOARD_ROW_LENGTH * BOARD_SIDE;

fn sum_unmarked(board: &[i64; BOARD_SIDE * BOARD_SIDE]) -> i64 {
    board.iter().filter(|&n| *n != -1).sum()
}

fn mark_number(board: &mut [i64; BOARD_SIDE * BOARD_SIDE], number: i64) -> bool {
    for i in 0..BOARD_SIDE {
        for j in 0..BOARD_SIDE {
            if board[i * BOARD_SIDE + j] != number {
                continue;
            }

            board[i * BOARD_SIDE + j] = -1;

            let mut score_x = 0;
            let mut score_y = 0;

            for k in 0..BOARD_SIDE {
                score_x += board[i * BOARD_SIDE + k];
                score_y += board[k * BOARD_SIDE + j];
            }

            return -score_x == BOARD_SIDE as i64 || -score_y == BOARD_SIDE as i64;
        }
    }

    false
}

pub fn run1(input: &[u8]) -> i64 {
    let numbers_raw = &input[..RAW_NUMBERS_LENGTH];
    // skip 2 newlines
    let boards_raw = &input[RAW_NUMBERS_LENGTH + 2..];

    let mut boards = [[0i64; BOARD_SIDE * BOARD_SIDE]; BOARD_COUNT];

    (0..BOARD_COUNT).for_each(|board_num| {
        for row_num in 0..BOARD_SIDE {
            // + 1 to account for extra newline between boards
            let start = board_num * (RAW_BOARD_LENGTH + 1) + row_num * RAW_BOARD_ROW_LENGTH;
            let row = &boards_raw[start..start + RAW_BOARD_ROW_LENGTH];

            for col_num in 0..BOARD_SIDE {
                let n = row[(col_num * 3) + 1] - b'0'
                    + if row[(col_num * 3)] != b' ' {
                        (row[(col_num * 3)] - b'0') * 10
                    } else {
                        0
                    };
                boards[board_num][row_num * BOARD_SIDE + col_num] = n as i64;
            }
        }
    });

    let mut pointer = 0;

    loop {
        if pointer >= RAW_NUMBERS_LENGTH {
            break;
        }

        let mut number = (numbers_raw[pointer] - b'0') as i64;

        if numbers_raw[pointer + 1] == b',' {
            pointer += 2;
        } else {
            number = number * 10 + (numbers_raw[pointer + 1] - b'0') as i64;
            pointer += 3;
        };

        for board in &mut boards {
            if mark_number(board, number) {
                return sum_unmarked(board) * number;
            }
        }
    }

    -1
}

// this fails on some inputs, i dont know why
#[allow(clippy::mut_range_bound)]
pub fn run2(input: &[u8]) -> i64 {
    let numbers_raw = &input[..RAW_NUMBERS_LENGTH];
    // skip 2 newlines
    let boards_raw = &input[RAW_NUMBERS_LENGTH + 2..];

    let mut boards = [[0i64; BOARD_SIDE * BOARD_SIDE]; BOARD_COUNT];

    (0..BOARD_COUNT).for_each(|board_num| {
        for row_num in 0..BOARD_SIDE {
            // + 1 to account for extra newline between boards
            let start = board_num * (RAW_BOARD_LENGTH + 1) + row_num * RAW_BOARD_ROW_LENGTH;
            let row = &boards_raw[start..start + RAW_BOARD_ROW_LENGTH];

            for col_num in 0..BOARD_SIDE {
                let n = row[(col_num * 3) + 1] - b'0'
                    + if row[(col_num * 3)] != b' ' {
                        (row[(col_num * 3)] - b'0') * 10
                    } else {
                        0
                    };
                boards[board_num][row_num * BOARD_SIDE + col_num] = n as i64;
            }
        }
    });

    let mut remaining_boards = BOARD_COUNT;

    let mut pointer = 0;

    loop {
        if pointer >= RAW_NUMBERS_LENGTH {
            break;
        }

        let mut number = (numbers_raw[pointer] - b'0') as i64;

        if numbers_raw[pointer + 1] == b',' {
            pointer += 2;
        } else {
            number = number * 10 + (numbers_raw[pointer + 1] - b'0') as i64;
            pointer += 3;
        };

        let mut offset = 0;

        for i in 0..remaining_boards {
            if mark_number(&mut boards[i], number) {
                if remaining_boards == 1 {
                    return sum_unmarked(&boards[i]) * number;
                }

                remaining_boards -= 1;

                continue;
            }

            boards[offset] = boards[i];

            offset += 1;
        }
    }

    -1
}

fn main() {
    let input = &fs::read("input").unwrap();

    println!("{}", run1(input));
    println!("{}", run2(input));
}
