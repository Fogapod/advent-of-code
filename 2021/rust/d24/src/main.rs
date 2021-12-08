fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d24::run1       {}", d24::run1(input));
    println!("d24::run2       {}", d24::run2(input));
}