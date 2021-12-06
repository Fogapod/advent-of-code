#![feature(portable_simd)]

use core_simd::*;

const FISH_COUNT: usize = 300;

const SIMD_U8_BATCH_SIZE: usize = 512 / 8;
const SIMD_U8_BATCHES_COUNT: usize = (FISH_COUNT * 2) / SIMD_U8_BATCH_SIZE;

// generated by python script
const SOLUTION_MAP_1: [i64; 10] = [1421, 1401, 1191, 1154, 1034, 950, 905, 779, 768, 642];
const SOLUTION_MAP_2: [i64; 10] = [
    6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552,
    3369186778, 3053201612,
];

// around 50% slower
pub fn run1_simd(input: &[u8]) -> i64 {
    let mut sum = 0;

    let simd_offset = u8x64::splat(b'0');

    let mut pointer = 0;

    for _ in 0..SIMD_U8_BATCHES_COUNT {
        let vector = u8x64::from_slice(&input[pointer..pointer + SIMD_U8_BATCH_SIZE]) - simd_offset;

        sum += (0..SIMD_U8_BATCH_SIZE / 2)
            .map(|j| SOLUTION_MAP_1[vector[j * 2] as usize])
            .sum::<i64>();

        pointer += SIMD_U8_BATCH_SIZE;
    }

    // 26 remaining bytes
    sum += (0..12)
        .map(|i| input[576 + i * 2] - b'0')
        .map(|n| SOLUTION_MAP_1[n as usize])
        .sum::<i64>();

    sum
}

pub fn run1(input: &[u8]) -> i64 {
    unsafe {
        (0..FISH_COUNT)
            .map(|i| input.get_unchecked(i * 2) - b'0')
            .map(|n| SOLUTION_MAP_1.get_unchecked(n as usize))
            .sum()
    }
}

pub fn run2(input: &[u8]) -> i64 {
    unsafe {
        (0..FISH_COUNT)
            .map(|i| input.get_unchecked(i * 2) - b'0')
            .map(|n| SOLUTION_MAP_2.get_unchecked(n as usize))
            .sum()
    }
}
