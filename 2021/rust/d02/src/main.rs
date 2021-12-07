fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d02::run1(input));
    println!("{}", d02::run2(input));
}
