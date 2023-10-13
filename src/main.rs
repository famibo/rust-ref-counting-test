use std::cell::{Cell, RefCell};
use std::cmp::Ordering::Equal;
use std::rc::{Rc, Weak};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
struct Person {
    _id: String,
    name: String,
    active: Cell<bool>,
    metier: RefCell<Rc<String>>,
    age: Cell<i32>,
    history: RefCell<Vec<Rc<String>>>,
    deputy: RefCell<Option<Rc<Person>>>,
    team: RefCell<Weak<Team>>,
}
#[derive(Debug)]
struct Team {
    _id: String,
    name: String,
    members: RefCell<Vec<Rc<Person>>>,
}

impl Person {
    pub fn new(name: &str, metier: &str, age: i32, deputy: Option<Rc<Person>>) -> Rc<Person> {
        Rc::new( Person {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            active: Cell::new(true),
            metier: RefCell::new(Rc::new(metier.to_string())),
            age: Cell::new(age),
            history: RefCell::new(vec!()),
            deputy: RefCell::new(deputy),
            team: RefCell::new(Weak::new()),
        })
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
impl fmt::Display for Person  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(team) = self.team.borrow().upgrade() {
            write!(f, "{:?} from team {}", self, team.name)
        } else {
            write!(f, "{:?} has NO TEAM ASSIGNED", self)
        }
    }
}
impl Team {
    pub fn new(name: &str) -> Rc<Team> {
        Rc::new(Team {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            members: RefCell::new(vec!()),
        })
    }
    fn add_member(self: &Rc<Self>, member: Rc<Person>) {
        for m in self.members.borrow().iter() {
            if m._id == member._id {
                //already team member -> exit and don't add again
                return
            }
        }
        *member.team.borrow_mut() = Rc::downgrade(self);
        self.members.borrow_mut().push(member);
    }
    fn find_member_by_metier(&self, metier: &str) -> Option<Rc<Person>> {
        if let Some(c) = self.members.borrow().iter().find(
            |m|
                m.metier.borrow().to_string().cmp(&metier.to_string()) == Equal && m.active.get() == true
            ) {
            Some(Rc::clone(c))
        }
        else {
            Option::None
        }
    }
    fn find_member_by_name(&self, name: &str) -> Option<Rc<Person>> {
        if let Some(c) = self.members.borrow().iter().find(
            |m| m.name.cmp(&name.to_string()) == Equal && m.active.get() == true) {
            Some(Rc::clone(c))
        }
        else {
            Option::None
        }
    }
    fn remove_member_by_name(&self, name: &str)  {
        self.members.borrow_mut().retain(|m| m.name.cmp(&name.to_string()) != Equal)
    }
    fn deactivate_member_by_name(&self, name: &str) {
        if let Some(c) = self.find_member_by_name(name) {
            c.active.set(false);
        }
    }
}
impl fmt::Display for Team  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn main() {
    let d = Person::new ("mccarthy","boss", 60, None);
    let p = Person::new ("employee","writer", 33, Some(Rc::clone(&d)));
    let team = Team::new("buddies");
    team.add_member(Rc::clone(&d));
    team.add_member(Rc::clone(&p));
    p.set_metier("programmer");
    p.set_age(44);
    p.set_metier("cyclist");
    println!("{}", d);
    println!("{}", p);
    println!();
    println!("{}", team);
    println!();
    //team.deactivate_member_by_name("employee");
    if let Some(c) = team.find_member_by_metier("cyclist") {
        println!("Found {}", c);
    }
    println!();
    println!("{}", team);
}
