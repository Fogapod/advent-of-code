fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d20::run1       {}", d20::run1(input));
    println!("d20::run2       {}", d20::run2(input));
}
