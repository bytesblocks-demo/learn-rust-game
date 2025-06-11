use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    println!("Hello, {}!", your_name.trim());
}
