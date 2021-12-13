fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();
    let input_str = &String::from_utf8(input.to_vec()).unwrap();

    println!("d13::run1       {}", d13::run1(input_str));
    println!("d13::run2       {}", d13::run2(input));
}
