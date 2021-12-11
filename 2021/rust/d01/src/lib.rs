#![feature(unchecked_math)]

const MIN_NUMBER_LEN: usize = 3;

const LINE_COUNT: usize = 2000;

pub fn run1(input: &[u8]) -> i64 {
    let mut counter: i64 = 0;

    let mut pointer: usize = 4;

    unsafe {
        let mut current = (input[2] - b'0') as i64
            + (input[1] - b'0') as i64 * 10
            + (input[0] - b'0') as i64 * 100;

        for _ in 1..LINE_COUNT {
            let mut next = ((input
                .get_unchecked(pointer.unchecked_add(2))
                .unchecked_sub(b'0')) as i64)
                .unchecked_add(
                    ((input
                        .get_unchecked(pointer.unchecked_add(1))
                        .unchecked_sub(b'0')) as i64)
                        .unchecked_mul(10)
                        .unchecked_add(
                            ((input.get_unchecked(pointer).unchecked_sub(b'0')) as i64)
                                .unchecked_mul(100),
                        ),
                );

            pointer = pointer.unchecked_add(MIN_NUMBER_LEN);

            loop {
                let byte = input[pointer];
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                next = next
                    .unchecked_mul(10)
                    .unchecked_add((byte.unchecked_sub(b'0')) as i64);
            }

            if next > current {
                counter = counter.unchecked_add(1);
            }

            current = next;
        }
    }

    counter
}

pub fn run2(input: &[u8]) -> i64 {
    const WINDOW_SIZE: usize = 3;

    let mut counter: i64 = 0;

    let mut window: i64 = 0;

    let mut pointer: usize = 0;

    let mut window_numbers = [0i64; 3];

    unsafe {
        (0..WINDOW_SIZE).for_each(|i| {
            let mut number: i64 = 0;

            loop {
                let byte = input[pointer];
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                number = number
                    .unchecked_mul(10)
                    .unchecked_add((byte.unchecked_sub(b'0')) as i64);
            }

            window_numbers[i] = number;

            window = window.unchecked_add(number);
        });

        for _ in WINDOW_SIZE..LINE_COUNT {
            let mut number = ((input
                .get_unchecked(pointer.unchecked_add(2))
                .unchecked_sub(b'0')) as i64)
                .unchecked_add(
                    ((input
                        .get_unchecked(pointer.unchecked_add(1))
                        .unchecked_sub(b'0')) as i64)
                        .unchecked_mul(10)
                        .unchecked_add(
                            ((input.get_unchecked(pointer).unchecked_sub(b'0')) as i64)
                                .unchecked_mul(100),
                        ),
                );

            pointer = pointer.unchecked_add(MIN_NUMBER_LEN);

            loop {
                let byte = input[pointer];
                pointer = pointer.unchecked_add(1);

                if byte == b'\n' {
                    break;
                }

                number = number
                    .unchecked_mul(10)
                    .unchecked_add((byte.unchecked_sub(b'0')) as i64);
            }

            let new_window = window - window_numbers[0] + number;

            if new_window > window {
                counter = counter.unchecked_add(1);
            }

            window_numbers[0] = window_numbers[1];
            window_numbers[1] = window_numbers[2];
            window_numbers[2] = number;

            window = new_window;
        }
    }

    counter
}
