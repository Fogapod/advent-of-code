// 10 for example, 90+ for full, assume 100 max?
const LINE_COUNT: usize = 100;

const LINE_SIZE: usize = 128; // assumption

// < >, [ ] and { } are 2 values apart while ( ) are 1 value
const OPENED_TO_CLOSED_MAP: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 0,
    0, 0, 0,
];

#[inline(always)]
fn is_opening(byte: u8) -> bool {
    byte == b'(' || byte == b'[' || byte == b'{' || byte == b'<'
}

pub fn run1(input: &[u8]) -> i64 {
    const ILLEGAL_SCORES: [i64; 128] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 25137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1197, 0, 0,
    ];

    let mut answer = 0;

    let input_len = input.len();

    let mut stack = [0; LINE_SIZE];

    let mut pointer = 0;

    unsafe {
        loop {
            if pointer == input_len {
                break;
            }

            let mut stack_size = 0;

            'line_scanner: loop {
                let byte = *input.get_unchecked(pointer);

                pointer += 1;

                if byte == b'\n' {
                    break;
                }

                if is_opening(byte) {
                    *stack.get_unchecked_mut(stack_size) = byte;

                    stack_size += 1;

                    continue;
                }

                stack_size -= 1;
                let popped = *stack.get_unchecked(stack_size);

                if is_opening(popped) && OPENED_TO_CLOSED_MAP[popped as usize] != byte {
                    answer += ILLEGAL_SCORES[byte as usize];

                    loop {
                        let byte = input[pointer];

                        pointer += 1;

                        if byte == b'\n' {
                            break 'line_scanner;
                        }
                    }
                }
            }
        }
    }

    answer
}

pub fn run2(input: &[u8]) -> i64 {
    const CHARACTER_COSTS: [i64; 128] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 0, 0, 0, 0,
    ];

    let input_len = input.len();

    let mut score_count = 0;
    let mut scores = [0; LINE_COUNT - 1];

    let mut pointer = 0;

    let mut stack = [0; LINE_SIZE];

    unsafe {
        loop {
            if pointer == input_len {
                break;
            }
            let mut valid = true;

            let mut stack_size = 0;

            'line_scanner: loop {
                let byte = *input.get_unchecked(pointer);

                pointer += 1;

                if byte == b'\n' {
                    break;
                }

                if is_opening(byte) {
                    *stack.get_unchecked_mut(stack_size) = byte;

                    stack_size += 1;

                    continue;
                }

                stack_size -= 1;
                let popped = *stack.get_unchecked(stack_size);

                if is_opening(popped) && OPENED_TO_CLOSED_MAP[popped as usize] != byte {
                    valid = false;

                    loop {
                        let byte = input[pointer];

                        pointer += 1;

                        if byte == b'\n' {
                            break 'line_scanner;
                        }
                    }
                }
            }

            if !valid {
                continue;
            }

            let mut score = 0;

            for i in (0..stack_size).rev() {
                score = score * 5 + CHARACTER_COSTS[stack[i] as usize]
            }

            scores[score_count] = score;
            score_count += 1;
        }

        scores[..score_count].sort_unstable();

        *scores.get_unchecked(score_count / 2)
    }
}
