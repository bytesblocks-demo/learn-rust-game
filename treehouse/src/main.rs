use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_theme_in = false;
    for visitor in visitor_list {
        if visitor == your_name {
            allow_theme_in = true;
            break;
        }
    }
    if allow_theme_in {
        println!("Welcome, {}!", your_name);
    } else {
        println!("Sorry, {}! You are not allowed in.", your_name);
    }
}
