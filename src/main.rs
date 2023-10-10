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
    d.print_person();
    p.set_metier("programmer");
    p.set_age(44);
    p.set_metier("cyclist");
    p.print_person();
}
