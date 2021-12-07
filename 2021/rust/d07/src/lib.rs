#![feature(int_abs_diff)]

use arrayvec::ArrayVec;

const NUMBER_COUNT: usize = 1000;

pub fn run1_brute(input: &[u8]) -> i64 {
    // trim newline
    let len = input.len();
    let input = unsafe { &input[..len - (*input.get_unchecked(len - 1) == b'\n') as usize] };

    let mut sorted_numbers = input
        .split(|&byte| byte == b',')
        .map(|raw_number| {
            raw_number
                .iter()
                .rev()
                .enumerate()
                .map(|(mut i, byte)| {
                    let mut byte = (byte - b'0') as i64;

                    while i > 0 {
                        i -= 1;
                        byte *= 10
                    }

                    byte
                })
                .sum()
        })
        .collect::<ArrayVec<i64, NUMBER_COUNT>>();

    sorted_numbers.sort_unstable();

    let (lowest, highest) = unsafe {
        (
            sorted_numbers.get_unchecked(0),
            sorted_numbers.get_unchecked(NUMBER_COUNT - 1),
        )
    };

    let mut lowest_dist = i64::MAX;

    // make sure to include highest number
    for i in *lowest..=*highest {
        let dist = sorted_numbers.iter().map(|n| n.abs_diff(i) as i64).sum();

        if dist < lowest_dist {
            lowest_dist = dist;
        }
    }

    lowest_dist
}

pub fn run1(input: &[u8]) -> i64 {
    // trim newline
    let len = input.len();
    let input = unsafe { &input[..len - (*input.get_unchecked(len - 1) == b'\n') as usize] };

    let mut sorted_numbers = input
        .split(|&byte| byte == b',')
        .map(|raw_number| {
            raw_number
                .iter()
                .rev()
                .enumerate()
                .map(|(mut i, byte)| {
                    let mut byte = (byte - b'0') as i64;

                    while i > 0 {
                        i -= 1;
                        byte *= 10
                    }

                    byte
                })
                .sum()
        })
        .collect::<ArrayVec<i64, NUMBER_COUNT>>();

    sorted_numbers.sort_unstable();

    let median = unsafe {
        (sorted_numbers.get_unchecked(NUMBER_COUNT / 2)
            + sorted_numbers.get_unchecked(NUMBER_COUNT / 2))
            / 2
    };

    sorted_numbers
        .iter()
        .map(|n| n.abs_diff(median) as i64)
        .sum()
}

#[inline]
fn triangular_number(n: i64) -> i64 {
    n * (n + 1) / 2
}

// FIXME: produces wrong result for example, but works for everything else
pub fn run2(input: &[u8]) -> i64 {
    // trim newline
    let len = input.len();
    let input = unsafe { &input[..len - (*input.get_unchecked(len - 1) == b'\n') as usize] };

    let numbers = input
        .split(|&byte| byte == b',')
        .map(|raw_number| {
            raw_number
                .iter()
                .rev()
                .enumerate()
                .map(|(mut i, byte)| {
                    let mut byte = (byte - b'0') as i64;

                    while i > 0 {
                        i -= 1;
                        byte *= 10
                    }

                    byte
                })
                .sum()
        })
        .collect::<ArrayVec<i64, NUMBER_COUNT>>();

    let mean: i64 = numbers.iter().sum::<i64>() / NUMBER_COUNT as i64;

    numbers
        .iter()
        .map(|n| triangular_number(n.abs_diff(mean) as i64))
        .sum()
}
