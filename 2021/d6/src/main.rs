use d6;

fn main() {
    const INPUT: &[u8] = include_bytes!("../input");

    println!("{}", d6::run1(INPUT));
    println!("{}", d6::run2(INPUT));
}
