fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d01::run1: {}", d01::run1(input));
    println!("d01::run2: {}", d01::run2(input));
}
