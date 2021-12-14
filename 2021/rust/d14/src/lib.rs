#![feature(slice_split_at_unchecked)]

use itertools::Itertools;

// for example: 4, 4, 16
// for full: 10, 20, 100
const UNIQUE_CHARACTERS: usize = 10;
const INITIAL_STRING_LEN: usize = 20;
const RULE_COUNT: usize = 100;

const LETTER_TRANSLATION_MAP: [usize; b'Z' as usize] = {
    let mut array = [100; b'Z' as usize];

    // example characters
    array[b'B' as usize] = 0;
    array[b'C' as usize] = 1;
    array[b'H' as usize] = 2;
    array[b'N' as usize] = 3;
    // full characters
    array[b'F' as usize] = 4;
    array[b'K' as usize] = 5;
    array[b'O' as usize] = 6;
    array[b'S' as usize] = 7;
    array[b'V' as usize] = 8;
    array[b'P' as usize] = 9;

    array
};

// for debugging
// const CODE_TRANSLATION_MAP: [u8; b'Z' as usize] = {
//     let mut array = [100; b'Z' as usize];

//     // example characters
//     array[0] = b'B';
//     array[1] = b'C';
//     array[2] = b'H';
//     array[3] = b'N';
//     // full characters
//     array[4] = b'F';
//     array[5] = b'K';
//     array[6] = b'O';
//     array[7] = b'S';
//     array[8] = b'V';
//     array[9] = b'P';

//     array
// };

pub fn run1(input: &[u8]) -> i64 {
    const ITERATIONS: usize = 10;

    let mut cache = [
        [
            [
                [
                    0;
                    // actual counter
                    UNIQUE_CHARACTERS
                ];
                // for each a
                UNIQUE_CHARACTERS
            ];
            // for each b
            UNIQUE_CHARACTERS
        ];
        // for each iteration
        ITERATIONS + 1
    ];

    let mut initial_string = [0; INITIAL_STRING_LEN];

    let mut rules = [[0usize; UNIQUE_CHARACTERS]; UNIQUE_CHARACTERS];

    let mut pointer = 0;

    unsafe {
        for i in 0..INITIAL_STRING_LEN {
            initial_string[i] = LETTER_TRANSLATION_MAP[input[i] as usize];
        }

        pointer += INITIAL_STRING_LEN + 2;

        for _ in 0..RULE_COUNT {
            rules[LETTER_TRANSLATION_MAP[input[pointer] as usize] as usize]
                [LETTER_TRANSLATION_MAP[input[pointer + 1] as usize] as usize] =
                LETTER_TRANSLATION_MAP[input[pointer + 6] as usize];

            pointer += 8;
        }

        for values in (0..UNIQUE_CHARACTERS).permutations(2) {
            cache[0][values[0] as usize][values[1] as usize][values[0] as usize] = 1;
        }

        // permutations skip these
        for i in 0..UNIQUE_CHARACTERS {
            cache[0][i][i][i] = 1;
        }

        for depth in 1..ITERATIONS + 1 {
            for values in (0..UNIQUE_CHARACTERS).permutations(2) {
                let k = rules[values[0] as usize][values[1] as usize];

                let mut left = cache[depth - 1][values[0]][k as usize];

                for (i, value) in cache[depth - 1][k as usize][values[1]].iter().enumerate() {
                    left[i] += value;
                }

                cache[depth][values[0] as usize][values[1] as usize] = left;
            }

            // permutations skip these
            (0..UNIQUE_CHARACTERS).for_each(|i| {
                let k = rules[i][i];

                let mut left = cache[depth - 1][i][k as usize];

                for (j, value) in cache[depth - 1][k as usize][i].iter().enumerate() {
                    left[j] += value;
                }

                cache[depth][i][i] = left;
            });
        }

        let mut counter = [0; UNIQUE_CHARACTERS];

        for i in 0..INITIAL_STRING_LEN - 1 {
            for (i, value) in cache[ITERATIONS][initial_string[i] as usize]
                [initial_string[i + 1] as usize]
                .iter()
                .enumerate()
            {
                counter[i] += value;
            }
        }

        counter[initial_string[INITIAL_STRING_LEN - 1] as usize] += 1;

        counter.sort_unstable();

        (counter[UNIQUE_CHARACTERS - 1] - counter[0]) as i64
    }
}

pub fn run2(input: &[u8]) -> i64 {
    const ITERATIONS: usize = 40;

    let mut cache = [
        [
            [
                [
                    0usize;
                    // actual counter
                    UNIQUE_CHARACTERS 
                ];
                // for each a
                UNIQUE_CHARACTERS
            ];
            // for each b
            UNIQUE_CHARACTERS
        ];
        // for each iteration
        ITERATIONS + 1
    ];

    let mut initial_string = [0usize; INITIAL_STRING_LEN];

    let mut rules = [[0usize; UNIQUE_CHARACTERS]; UNIQUE_CHARACTERS];

    let mut pointer = 0;

    unsafe {
        for i in 0..INITIAL_STRING_LEN {
            initial_string[i] = LETTER_TRANSLATION_MAP[input[i] as usize];
        }

        pointer += INITIAL_STRING_LEN + 2;

        for _ in 0..RULE_COUNT {
            rules[LETTER_TRANSLATION_MAP[input[pointer] as usize] as usize]
                [LETTER_TRANSLATION_MAP[input[pointer + 1] as usize] as usize] =
                LETTER_TRANSLATION_MAP[input[pointer + 6] as usize];

            pointer += 8;
        }

        for values in (0..UNIQUE_CHARACTERS).permutations(2) {
            cache[0][values[0] as usize][values[1] as usize][values[0] as usize] = 1;
        }

        // permutations skip these
        for i in 0..UNIQUE_CHARACTERS {
            cache[0][i][i][i] = 1;
        }

        for depth in 1..ITERATIONS + 1 {
            for values in (0..UNIQUE_CHARACTERS).permutations(2) {
                let k = rules[values[0] as usize][values[1] as usize];

                let mut left = cache[depth - 1][values[0]][k as usize];

                for (i, value) in cache[depth - 1][k as usize][values[1]].iter().enumerate() {
                    left[i] += value;
                }

                cache[depth][values[0] as usize][values[1] as usize] = left;
            }

            // permutations skip these
            (0..UNIQUE_CHARACTERS).for_each(|i| {
                let k = rules[i][i];

                let mut left = cache[depth - 1][i][k as usize];

                for (j, value) in cache[depth - 1][k as usize][i].iter().enumerate() {
                    left[j] += value;
                }

                cache[depth][i][i] = left;
            });
        }

        let mut counter = [0; UNIQUE_CHARACTERS];

        for i in 0..INITIAL_STRING_LEN - 1 {
            for (i, value) in cache[ITERATIONS][initial_string[i] as usize]
                [initial_string[i + 1] as usize]
                .iter()
                .enumerate()
            {
                counter[i] += value;
            }
        }

        counter[initial_string[INITIAL_STRING_LEN - 1] as usize] += 1;

        counter.sort_unstable();

        (counter[UNIQUE_CHARACTERS - 1] - counter[0]) as i64
    }
}
