#![feature(unchecked_math)]

const FISH_COUNT: usize = 300;

// generated by python script
const SOLUTION_MAP_1: [i64; 58] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1421, 1401, 1191, 1154, 1034, 950, 905, 779,
    768, 642,
];
const SOLUTION_MAP_2: [i64; 58] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6703087164, 6206821033, 5617089148, 5217223242,
    4726100874, 4368232009, 3989468462, 3649885552, 3369186778, 3053201612,
];

pub fn run1(input: &[u8]) -> i64 {
    unsafe {
        (0..FISH_COUNT)
            .map(|i| *input.get_unchecked(i.unchecked_mul(2)))
            .map(|n| SOLUTION_MAP_1.get_unchecked(n as usize))
            .sum()
    }
}

pub fn run2(input: &[u8]) -> i64 {
    unsafe {
        (0..FISH_COUNT)
            .map(|i| *input.get_unchecked(i.unchecked_mul(2)))
            .map(|n| SOLUTION_MAP_2.get_unchecked(n as usize))
            .sum()
    }
}
