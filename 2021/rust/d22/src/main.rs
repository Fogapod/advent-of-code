fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d22::run1       {}", d22::run1(input));
    println!("d22::run2       {}", d22::run2(input));
}
