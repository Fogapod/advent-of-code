use std::fs;

const BOARD_SIDE: usize = 5;
const BOARD_COUNT: usize = 100;

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

            if score_x == -(BOARD_SIDE as i64) || score_y == -(BOARD_SIDE as i64) {
                return true;
            }

            break;
        }
    }

    false
}

pub fn run1(input: &str) -> i64 {
    let (numbers, rest) = input.split_once("\n\n").unwrap();

    let mut boards = [[0i64; BOARD_SIDE * BOARD_SIDE]; BOARD_COUNT];

    for (board_num, board) in rest.split("\n\n").enumerate() {
        for (row_num, line) in board.split('\n').enumerate() {
            for (col_num, n) in line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .enumerate()
            {
                boards[board_num][row_num * BOARD_SIDE + col_num] = n;
            }
        }
    }

    for number in numbers.split(',').map(|n| n.parse::<i64>().unwrap()) {
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
pub fn run(input: &str) -> i64 {
    let (numbers, rest) = input.split_once("\n\n").unwrap();

    let mut boards = [[0i64; BOARD_SIDE * BOARD_SIDE]; BOARD_COUNT];

    for (board_num, board) in rest.split("\n\n").enumerate() {
        for (row_num, line) in board.split('\n').enumerate() {
            for (col_num, n) in line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .enumerate()
            {
                boards[board_num][row_num * BOARD_SIDE + col_num] = n;
            }
        }
    }

    let mut remaining_boards = BOARD_COUNT;

    for number in numbers.split(',').map(|n| n.parse::<i64>().unwrap()) {
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
    let input = &fs::read_to_string("input").unwrap();

    println!("{}", run1(input));
    println!("{}", run(input));
}
