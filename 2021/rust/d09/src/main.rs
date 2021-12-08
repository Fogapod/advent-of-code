fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d09::run1       {}", d09::run1(input));
    println!("d09::run2       {}", d09::run2(input));
}