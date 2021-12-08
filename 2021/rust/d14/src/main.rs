fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d14::run1       {}", d14::run1(input));
    println!("d14::run2       {}", d14::run2(input));
}