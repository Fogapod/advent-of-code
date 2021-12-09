fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d18::run1       {}", d18::run1(input));
    println!("d18::run2       {}", d18::run2(input));
}
