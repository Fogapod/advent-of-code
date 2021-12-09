fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d16::run1       {}", d16::run1(input));
    println!("d16::run2       {}", d16::run2(input));
}
