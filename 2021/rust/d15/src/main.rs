fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d15::run1       {}", d15::run1(input));
    println!("d15::run2       {}", d15::run2(input));
}
