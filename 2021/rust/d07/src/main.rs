use d07;

fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d07::run1(input));
    println!("{}", d07::run2(input));
}
