fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d25::run1       {}", d25::run1(input));
    println!("d25::run2       {}", d25::run2(input));
}
