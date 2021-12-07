fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d07::run1_brute {}", d07::run1_brute(input));
    println!("d07::run1       {}", d07::run1(input));
    println!("d07::run2       {}", d07::run2(input));
}
