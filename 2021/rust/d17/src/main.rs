fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d17::run1       {}", d17::run1(input));
    println!("d17::run2       {}", d17::run2(input));
}