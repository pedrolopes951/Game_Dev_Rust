use std::io::stdin;
#[derive(Debug)]
struct Visitor {
    name: String,
    age : i8,
    action: Bouncer,
}

#[derive(Debug)]
enum Bouncer{
    Accepted,
    Refused,
    AcceptedWithNote {note:String},
    Probation,
}
impl Visitor {
    fn new(name: &str, action: Bouncer, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        println!("{:#?}", self);
        match &self.action {// (8)
        Bouncer::Accepted => println!("Welcome to the Tree House, {}", self.name),
        Bouncer::AcceptedWithNote { note } =>{ println!("Welcome to the tree
        house, {}",self.name);
        println!("{}",note);
        if self.age < 18 {
            println!("Do not serve alcool to {}", self.name);
        }
        },
        Bouncer::Refused => println!("Sorry, {} can't come in", self.name),
        Bouncer::Probation => println!("{} is prohibited to enter here", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
       
    let mut visitor_list = vec![Visitor::new("Bert",Bouncer::Accepted,45),
    Visitor::new("Steve",Bouncer::AcceptedWithNote { note: String::from("Lactose-free milk is in the fridge")},
     15),Visitor::new("Fred", Bouncer::Refused, 30),];
 loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {    // (1)
                    break;  // (2)
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name,Bouncer::Probation,0));
                }
            }
        }
    }
    print!("The final visitor list \n {:#?}", visitor_list);
}