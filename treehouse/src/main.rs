use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line.");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut allow_them_in = false;
    let visitor_list = ["bert", "steve", "fred"];
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    for visitor in &visitor_list {
        if visitor == &your_name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Hello, {}", your_name)
    } else {
        println!("Sorry you aren't on the allow list.")
    }
}
