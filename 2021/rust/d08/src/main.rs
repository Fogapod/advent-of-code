fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d08::run1       {}", d08::run1(input));
    println!("d08::run2       {}", d08::run2(input));
}
