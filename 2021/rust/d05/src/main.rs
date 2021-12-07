use d05;

fn main() {
    let input = &std::fs::read_to_string(format!("../input/{}/full", module_path!())).unwrap();

    println!("{}", d05::run1(input));
    println!("{}", d05::run2(input));
}
