// --- parser related constants --- //
// 10 for example, 200 for full
const LINE_COUNT: usize = 200;

const RAW_INPUTS_LEN: usize = 58;

// skip " | "
const OFFSET_UNTIL_OUTPUTS: usize = RAW_INPUTS_LEN + 3;

// 3 newlines + 1 (2 chars) 4 times
const MIN_OUTPUTS_LEN: usize = 3 + 2 * 4;

// 1 (2 chars)
const MIN_NUM_LEN: usize = 2;
// --- parser related constants --- //

pub fn run1(input: &[u8]) -> i64 {
    const VALID_LENGTHS_MAP: [i64; 10] = [0, 0, 1, 1, 1, 0, 0, 1, 0, 0];

    let mut sum = 0;

    // start at first outputs
    let mut pointer = OFFSET_UNTIL_OUTPUTS + MIN_OUTPUTS_LEN + 1;

    unsafe {
        for _ in 0..LINE_COUNT {
            let mut outputs_len = MIN_OUTPUTS_LEN + 1;

            while *input.get_unchecked(pointer) != b'\n' {
                pointer += 1;
                outputs_len += 1;
            }

            let outputs = &input[pointer - outputs_len..pointer];

            let mut num_pointer = MIN_NUM_LEN;
            let mut num_len = MIN_NUM_LEN;

            loop {
                if num_pointer == outputs_len {
                    sum += VALID_LENGTHS_MAP.get_unchecked(num_len);

                    break;
                }

                if outputs[num_pointer] == b' ' {
                    sum += VALID_LENGTHS_MAP.get_unchecked(num_len);

                    num_len = MIN_NUM_LEN;
                    num_pointer += 1 + MIN_NUM_LEN;
                } else {
                    num_pointer += 1;
                    num_len += 1;
                }
            }

            pointer += MIN_OUTPUTS_LEN + OFFSET_UNTIL_OUTPUTS + 2;
        }
    }

    sum
}

pub fn run2(input: &[u8]) -> i64 {
    const SEGMENTS_TO_VALUE_MAP: [i64; 128] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 7, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 3, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 6, 0, 0, 0, 8,
    ];

    const A: i8 = 1 << 0;
    const B: i8 = 1 << 1;
    const C: i8 = 1 << 2;
    const D: i8 = 1 << 3;
    const E: i8 = 1 << 4;
    const F: i8 = 1 << 5;
    const G: i8 = 1 << 6;

    let mut sum = 0;

    // start at first outputs
    let mut pointer = OFFSET_UNTIL_OUTPUTS + MIN_OUTPUTS_LEN + 1;

    unsafe {
        for _ in 0..LINE_COUNT {
            let mut inputs_and_outputs_len = OFFSET_UNTIL_OUTPUTS + MIN_OUTPUTS_LEN + 1;

            while *input.get_unchecked(pointer) != b'\n' {
                pointer += 1;
                inputs_and_outputs_len += 1;
            }

            let raw_inputs = &input[pointer - inputs_and_outputs_len
                ..pointer - inputs_and_outputs_len + RAW_INPUTS_LEN];

            let raw_outputs =
                &input[pointer - inputs_and_outputs_len + OFFSET_UNTIL_OUTPUTS..pointer];

            // avoid sorting by length by inserting items into array with already known offsets
            let mut inputs_offsets = [0, 0, -2, -2, -2, -2, 0, 2];
            let mut inputs = [0i8; 10];

            for raw_input in raw_inputs.split(|&byte| byte == b' ') {
                let offset = &mut inputs_offsets[raw_input.len()];

                inputs[(*offset + raw_input.len() as i8) as usize] =
                    raw_input.iter().map(|&byte| 1 << (byte - b'a')).sum();

                *offset += 1;
            }

            // see python solution for explanation of this part
            let one = inputs[0];
            let four = inputs[2];
            let seven = inputs[1];
            let eight = inputs[9];

            let mut decoding_table = [0i8; 128];

            let a = seven ^ one;

            decoding_table[a as usize] = A;

            let b_and_d = four ^ one;

            let c_and_d_and_e = eight ^ inputs[6] ^ inputs[7] ^ inputs[8];

            let d = b_and_d & c_and_d_and_e;

            decoding_table[d as usize] = D;
            decoding_table[(b_and_d ^ d) as usize] = B;

            let c_and_e = c_and_d_and_e ^ d;

            let c = one & c_and_e;

            decoding_table[c as usize] = C;

            let e = c_and_e ^ c;

            decoding_table[e as usize] = E;
            decoding_table[(one ^ c) as usize] = F;
            decoding_table[(eight ^ (four | e | a)) as usize] = G;

            let mut num: i64 = 0;

            for (i, raw_output) in raw_outputs.split(|&byte| byte == b' ').rev().enumerate() {
                let le = raw_output.len();

                let digit = match le {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => *SEGMENTS_TO_VALUE_MAP.get_unchecked(
                        raw_output
                            .iter()
                            .map(|&byte| {
                                *decoding_table.get_unchecked((1 << (byte - b'a')) as usize)
                                    as usize
                            })
                            .sum::<usize>(),
                    ),
                };

                num += digit * i64::pow(10, i as u32);
            }

            sum += num;

            pointer += MIN_OUTPUTS_LEN + OFFSET_UNTIL_OUTPUTS + 2;
        }
    }

    sum
}
