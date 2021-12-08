fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d13::run1       {}", d13::run1(input));
    println!("d13::run2       {}", d13::run2(input));
}