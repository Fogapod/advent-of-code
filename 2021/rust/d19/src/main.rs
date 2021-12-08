fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d19::run1       {}", d19::run1(input));
    println!("d19::run2       {}", d19::run2(input));
}