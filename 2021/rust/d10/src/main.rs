fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("d10::run1       {}", d10::run1(input));
    println!("d10::run2       {}", d10::run2(input));
}
