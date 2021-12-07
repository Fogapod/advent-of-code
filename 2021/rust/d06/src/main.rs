use d06;

fn main() {
    let input = &std::fs::read(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d06::run1(input));
    println!("{}", d06::run2(input));
}
