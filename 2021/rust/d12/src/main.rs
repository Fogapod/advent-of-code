fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d12::run1       {}", d12::run1(input));
    println!("d12::run2       {}", d12::run2(input));
}