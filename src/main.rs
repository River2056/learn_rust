use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name: String = String::new();
    stdin()
        .read_line(&mut name)
        .expect("error while reading name");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what is your name?");

    let name: String = what_is_your_name();

    let visitors: [&str; 3] = ["kevin", "river", "tung"];
    let mut allowed_in = false;

    for visitor_name in &visitors {
        if visitor_name == &name {
            allowed_in = true
        }
    }

    if allowed_in {
        println!("Welcome, {}, have a nice day!", name)
    } else {
        println!("Sorry, name not on the list!")
    }
}
