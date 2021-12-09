fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d21::run1       {}", d21::run1(input));
    println!("d21::run2       {}", d21::run2(input));
}
