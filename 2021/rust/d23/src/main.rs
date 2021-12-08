fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d23::run1       {}", d23::run1(input));
    println!("d23::run2       {}", d23::run2(input));
}