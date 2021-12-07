fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d04::run1(input));
    println!("{}", d04::run2(input));
}
