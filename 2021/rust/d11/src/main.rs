fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d11::run1       {}", d11::run1(input));
    println!("d11::run2       {}", d11::run2(input));
}
