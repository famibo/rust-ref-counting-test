use std::cell::{Cell, RefCell};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug)]
struct Person {
    id: String,
    metier: RefCell<String>,
    age: Cell<i32>,
    history:  RefCell<Vec<String>>
}

impl Person {
    pub fn new(metier: &str, age: i32) -> Person {
        Person {
            id: Uuid::new_v4().to_string(),
            metier: RefCell::new(metier.to_string()),
            age: Cell::new(age),
            history: RefCell::new(vec!())
        }
    }
    fn print_person(&self) {
        println!("{:?}", self);
    }
    fn set_metier(&self, metier: &str) {
        self.history.borrow_mut().push(self.metier.borrow().clone());
        *self.metier.borrow_mut() = metier.to_string();
    }
    fn set_age(&self, age: i32) {
        self.age.set(age);
    }
}

fn some_func(person: Rc<Person>) {
    println!("{:?}", person);
}

fn main() {
   let p = Rc::new( Person::new (
       "writer", 33
    ));
    p.print_person();
    p.set_metier("programmer");
    p.set_age(44);
    p.print_person();
    p.set_metier("cyclist");
    p.set_age(96);
    p.print_person();
}
