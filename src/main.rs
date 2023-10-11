use std::cell::RefCell;
use std::rc::{Rc, Weak};
use uuid::Uuid;

#[derive(Debug)]
struct Member {
    _id: String,
    name: String,
    team: RefCell<Option<Weak<Team>>>,
}
#[derive(Debug)]
struct Team {
    _id: String,
    name: String,
    members: RefCell<Vec<Rc<Member>>>,
}
impl Member {
    pub fn new(name: &str) -> Member {
        Member {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            team: RefCell::new(Option::None),
        }
    }
    fn print(&self) {
        println!("{:?} from team {}", self, self.team.borrow().as_ref().unwrap().upgrade().unwrap().name);
    }
}
impl Team {
    pub fn new(name: &str) -> Team {
        Team {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            members: RefCell::new(vec!()),
        }
    }
    fn print(&self) {
        println!("{:?}", self);
    }

    fn add_member(self: &Rc<Self>, member: Rc<Member>) {
        for m in self.members.borrow().iter() {
            if m._id == member._id {
                //already team member -> exit and don't add again
                return
            }
        }
        *member.team.borrow_mut() = Option::Some(Rc::downgrade(self));
        self.members.borrow_mut().push(member);
    }
}

fn main() {
    let a = Rc::new( Member::new("boss"));
    let b = Rc::new( Member::new("employee"));
    let team = Rc::new(Team::new("buddies"));
    team.add_member(Rc::clone(&a));
    team.add_member(Rc::clone(&b));
    a.print();
    b.print();
    println!();
    team.print();
}
