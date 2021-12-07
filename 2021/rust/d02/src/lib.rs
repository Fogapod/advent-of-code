const LINE_COUNT: usize = 1000;

const OFFSET_MAP: [usize; 118] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 5, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
];

const OFFSET_MAP_PLUS_2: [usize; 118] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 7, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
];

pub fn run1(input: &[u8]) -> i64 {
    let mut positions = [0; 118];

    let mut pointer = 0;

    unsafe {
        for _ in 0..LINE_COUNT {
            let byte = *input.get_unchecked(pointer);

            *positions.get_unchecked_mut(byte as usize) += (*input
                .get_unchecked(pointer + OFFSET_MAP.get_unchecked(byte as usize))
                - b'0') as i64;

            pointer += OFFSET_MAP_PLUS_2.get_unchecked(byte as usize);
        }

        positions.get_unchecked(b'f' as usize)
            * (positions.get_unchecked(b'd' as usize) - positions.get_unchecked(b'u' as usize))
    }
}

pub fn run2(input: &[u8]) -> i64 {
    let mut accum = 0;
    let mut positions = [0; 118];

    let mut pointer = 0;

    unsafe {
        for _ in 0..LINE_COUNT {
            let byte = *input.get_unchecked(pointer);

            let num = (input.get_unchecked(pointer + OFFSET_MAP.get_unchecked(byte as usize))
                - b'0') as i64;

            *positions.get_unchecked_mut(byte as usize) += num;

            if byte == b'f' {
                accum += (positions.get_unchecked(b'd' as usize)
                    - positions.get_unchecked(b'u' as usize))
                    * num;
            }

            pointer += OFFSET_MAP_PLUS_2.get_unchecked(byte as usize);
        }

        positions.get_unchecked(b'f' as usize) * accum
    }
}
