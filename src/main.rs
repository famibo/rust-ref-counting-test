use std::cell::{Cell, RefCell};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug)]
struct Person {
    _id: String,
    metier: RefCell<Rc<String>>,
    age: Cell<i32>,
    history:  RefCell<Vec<Rc<String>>>,
    deputy: RefCell<Option<Rc<Person>>>,
}
#[derive(Debug)]
struct Team {
    _id: String,
    members: RefCell<Vec<Rc<Person>>>,
}

impl Person {
    pub fn new(metier: &str, age: i32, deputy: Option<Rc<Person>>) -> Person {
        Person {
            _id: Uuid::new_v4().to_string(),
            metier: RefCell::new(Rc::new(metier.to_string())),
            age: Cell::new(age),
            history: RefCell::new(vec!()),
            deputy: RefCell::new(deputy)
        }
    }
    fn print_person(&self) {
        println!("{:?}", self);
    }
    fn set_metier(&self, metier: &str) {
        self.history.borrow_mut().push(Rc::clone(&self.metier.borrow()));
        *self.metier.borrow_mut() = Rc::new(metier.to_string());
    }
    fn set_age(&self, age: i32) {
        self.age.set(age);
    }
    fn set_deputy(&self, deputy: Rc<Person>) {
        *self.deputy.borrow_mut() = Option::Some(deputy);
    }
}

impl Team {
    pub fn new() -> Team {
        Team {
            _id: Uuid::new_v4().to_string(),
            members: RefCell::new(vec!()),
        }
    }
    fn print_team(&self) {
        println!("{:?}", self);
    }
    fn add_member(&self, member: Rc<Person>) {
        for m in self.members.borrow().iter() {
            if m._id == member._id {
                //already team member -> exit and don't add again
                return
            }
        }
        self.members.borrow_mut().push(member);
    }
}

fn _some_func(person: Rc<Person>) {
    println!("{:?}", person);
}

fn main() {
    let d = Rc::new( Person::new (
        "boss", 60, None
    ));
    let p = Rc::new( Person::new (
       "writer", 33, Some(Rc::clone(&d))
    ));
    let team = Team::new();
    team.add_member(Rc::clone(&d));
    team.add_member(Rc::clone(&p));
    //Trying to add an already existing member
    team.add_member(Rc::clone(&d));
    d.print_person();
    p.set_metier("programmer");
    p.set_age(44);
    p.set_metier("cyclist");
    p.print_person();
    println!("");
    team.print_team();
}
