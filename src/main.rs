use std::cell::{Cell, RefCell};
use std::cmp::Ordering::Equal;
use std::rc::{Rc, Weak};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
struct Member {
    _id: String,
    name: String,
    active: Cell<bool>,
    team: RefCell<Weak<Team>>,
}
#[derive(Debug)]
struct Team {
    _id: String,
    name: String,
    members: RefCell<Vec<Rc<Member>>>,
}
impl Member {
    pub fn new(name: &str) -> Rc<Member> {
        Rc::new(Member {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            active: Cell::new(true),
            team: RefCell::new(Weak::new()),
        })
    }
}
impl fmt::Display for Member  {
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
    fn add_member(self: &Rc<Self>, member: Rc<Member>) {
        for m in self.members.borrow().iter() {
            if m._id == member._id {
                //already team member -> exit and don't add again
                return
            }
        }
        *member.team.borrow_mut() = Rc::downgrade(self);
        self.members.borrow_mut().push(member);
    }
    fn find_member_by_name(&self, name: &str) -> Option<Rc<Member>> {
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
    let a = Member::new("boss");
    let b = Member::new("employee");
    let team = Team::new("buddies");
    team.add_member(Rc::clone(&a));
    team.add_member(Rc::clone(&b));
    println!("{}", a);
    println!("{}", b);
    println!();
    println!("{}", team);
    println!();
    team.deactivate_member_by_name("employee");
    if let Some(c) = team.find_member_by_name("employee") {
        println!("Found {}", c);
    }
    println!();
    println!("{}", team);
}