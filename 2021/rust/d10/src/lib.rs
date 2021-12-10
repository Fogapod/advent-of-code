#![feature(unchecked_math)]

// 10 for example, 90+ for full, assume 100 max?
const LINE_COUNT: usize = 100;

const LINE_SIZE: usize = 128; // assumption

// < >, [ ] and { } are 2 values apart while ( ) are 1 value, so offset cannot be used
const OPENED_TO_CLOSED_MAP: [u8; 128] = {
    let mut array = [0u8; 128];

    array[b'(' as usize] = b')';
    array[b'[' as usize] = b']';
    array[b'{' as usize] = b'}';
    array[b'<' as usize] = b'>';

    array
};

const OPENING_BRACKET_MAP: [bool; 128] = {
    let mut array = [false; 128];

    array[b'(' as usize] = true;
    array[b'[' as usize] = true;
    array[b'{' as usize] = true;
    array[b'<' as usize] = true;

    array
};

#[inline(always)]
const fn is_opening(byte: u8) -> bool {
    OPENING_BRACKET_MAP[byte as usize]
}

pub fn run1(input: &[u8]) -> i64 {
    const ILLEGAL_SCORES: [i64; 128] = {
        let mut array = [0i64; 128];

        array[b')' as usize] = 3;
        array[b']' as usize] = 57;
        array[b'}' as usize] = 1197;
        array[b'>' as usize] = 25137;

        array
    };

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

                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                if is_opening(byte) {
                    *stack.get_unchecked_mut(stack_size) = byte;
                    stack_size = stack_size.unchecked_add(1);

                    continue;
                }

                stack_size = stack_size.unchecked_sub(1);
                let popped = *stack.get_unchecked(stack_size);

                if is_opening(popped) && OPENED_TO_CLOSED_MAP[popped as usize] != byte {
                    answer += ILLEGAL_SCORES[byte as usize];

                    loop {
                        let byte = input[pointer];

                        pointer = pointer.unchecked_add(1);

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
    const CHARACTER_COSTS: [i64; 128] = {
        let mut array = [0i64; 128];

        array[b'(' as usize] = 1;
        array[b'[' as usize] = 2;
        array[b'{' as usize] = 3;
        array[b'<' as usize] = 4;

        array
    };

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

                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                if is_opening(byte) {
                    *stack.get_unchecked_mut(stack_size) = byte;
                    stack_size = stack_size.unchecked_add(1);

                    continue;
                }

                stack_size = stack_size.unchecked_sub(1);
                let popped = *stack.get_unchecked(stack_size);

                if is_opening(popped) && OPENED_TO_CLOSED_MAP[popped as usize] != byte {
                    valid = false;

                    loop {
                        let byte = input[pointer];

                        pointer = pointer.unchecked_add(1);

                        if byte == b'\n' {
                            break 'line_scanner;
                        }
                    }
                }
            }

            if !valid {
                continue;
            }

            let mut score: i64 = 0;

            for i in (0..stack_size).rev() {
                score = score
                    .unchecked_mul(5)
                    .unchecked_add(CHARACTER_COSTS[stack[i] as usize]);
            }

            scores[score_count] = score;
            score_count = score_count.unchecked_add(1);
        }

        scores[..score_count].sort_unstable();

        *scores.get_unchecked(score_count / 2)
    }
}
