use std::io::stdin;

#[path = "./enum_modules/visitor_action.rs"]
mod enum_modules;
use enum_modules::*;

#[derive(Debug)]
struct Visitor {
    name: String,
    visitor_action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, visitor_action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            visitor_action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.visitor_action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", &self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
        Visitor::new("kevin", VisitorAction::Accept, 30),
        Visitor::new(
            "river",
            VisitorAction::AcceptWithNote {
                note: "test".to_string(),
            },
            35,
        ),
        Visitor::new("tung", VisitorAction::Refuse, 16),
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
                    visitors.push(Visitor::new(&name, VisitorAction::Probation, 18))
                }
            }
        }
    }
    println!("the final list of visitors:");
    println!("{:#?}", visitors);
}
