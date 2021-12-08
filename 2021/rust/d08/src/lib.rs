// --- parser related constants --- //
// 10 for example, 200 for full
const LINE_COUNT: usize = 200;

const INPUTS_COUNT: usize = 10;
const OUTPUTS_COUNT: usize = 4;

const RAW_INPUTS_LEN: usize = 58;

// skip " | "
const OFFSET_UNTIL_OUTPUTS: usize = RAW_INPUTS_LEN + 3;

// 1 (2 chars)
const MIN_NUM_LEN: usize = 2;
// --- parser related constants --- //

pub fn run1(input: &[u8]) -> i64 {
    const VALID_LENGTHS_MAP: [i64; 8] = [0, 0, 1, 1, 1, 0, 0, 1];

    let mut sum = 0;

    // start at first outputs
    let mut pointer = OFFSET_UNTIL_OUTPUTS;

    unsafe {
        for _ in 0..LINE_COUNT {
            pointer += MIN_NUM_LEN;

            let mut num_len = MIN_NUM_LEN;

            loop {
                let byte = *input.get_unchecked(pointer);

                if byte == b'\n' {
                    sum += VALID_LENGTHS_MAP.get_unchecked(num_len);

                    break;
                } else if byte == b' ' {
                    sum += VALID_LENGTHS_MAP.get_unchecked(num_len);

                    num_len = MIN_NUM_LEN;
                    pointer += 1 + MIN_NUM_LEN;
                } else {
                    pointer += 1;
                    num_len += 1;
                }
            }

            pointer += OFFSET_UNTIL_OUTPUTS + 1;
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

    const A: usize = 1 << 0;
    const B: usize = 1 << 1;
    const C: usize = 1 << 2;
    const D: usize = 1 << 3;
    const E: usize = 1 << 4;
    const F: usize = 1 << 5;
    const G: usize = 1 << 6;

    let mut answer = 0;

    let mut pointer = 0;

    unsafe {
        for _ in 0..LINE_COUNT {
            // avoid sorting by length by inserting items into array with already known offsets
            let mut inputs_offsets = [0, 0, -2, -2, -2, -2, 0, 2];
            let mut inputs = [0; 10];

            for _ in 0..INPUTS_COUNT {
                let mut num = 0;
                let mut num_len = 0;

                loop {
                    let byte = *input.get_unchecked(pointer);

                    pointer += 1;

                    if byte == b' ' {
                        let offset = inputs_offsets.get_unchecked_mut(num_len as usize);

                        *inputs.get_unchecked_mut((*offset + num_len) as usize) = num;

                        *offset += 1;

                        break;
                    } else {
                        num |= 1 << (byte - b'a');
                        num_len += 1;
                    }
                }
            }

            // skip "| "
            pointer += 2;

            // see python solution for explanation of this part
            let one = inputs[0];
            let four = inputs[2];
            let seven = inputs[1];
            let eight = inputs[9];

            let mut decoding_table = [0; 128];

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

            let mut sum = 0;

            for i in (0..OUTPUTS_COUNT).rev() {
                let mut num = 0;
                let mut num_len = 0;

                loop {
                    let byte = *input.get_unchecked(pointer);

                    pointer += 1;

                    // both '\n' and ' '
                    if byte <= b' ' {
                        let digit = match num_len {
                            2 => 1,
                            3 => 7,
                            4 => 4,
                            7 => 8,
                            _ => *SEGMENTS_TO_VALUE_MAP.get_unchecked(num as usize),
                        };

                        sum += digit * i64::pow(10, i as u32);

                        break;
                    } else {
                        num |= *decoding_table.get_unchecked(1 << (byte - b'a'));
                        num_len += 1;
                    }
                }
            }

            answer += sum;
        }
    }

    answer
}
