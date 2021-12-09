fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d05::run1(input));
    println!("{}", d05::run2(input));
}
