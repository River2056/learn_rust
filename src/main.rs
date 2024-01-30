use std::io::stdin;

#[path = "./enum_modules/visitor_action.rs"]
mod enum_modules;
use enum_modules::*;

struct Visitor {
    name: String,
    visitor_action: VisitorAction,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str, visitor_action: VisitorAction) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            visitor_action,
        }
    }

    fn greet_visitor(&self) {
        println!("Hi {}, {}", self.name, self.greeting)
    }
}

fn what_is_your_name() -> String {
    let mut name: String = String::new();
    stdin()
        .read_line(&mut name)
        .expect("error while reading name");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitors: Vec<Visitor> = vec![
        Visitor::new("kevin", "Welcome back, master", VisitorAction::Accept),
        Visitor::new(
            "river",
            "player, what's up?",
            VisitorAction::AcceptWithNote {
                note: "test".to_string(),
            },
        ),
        Visitor::new(
            "tung",
            "Welcome, be careful of surroundings when entering",
            VisitorAction::Refuse,
        ),
    ];

    loop {
        println!("Hello, what is your name?");
        let name: String = what_is_your_name();
        let known_visitor: Option<&Visitor> = visitors.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("Sorry {}, you're not on the list", name);
                    visitors.push(Visitor::new(&name, "New Friend", VisitorAction::Probation))
                }
            }
        }
    }
}
