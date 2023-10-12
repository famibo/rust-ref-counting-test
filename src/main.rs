use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
struct Member {
    _id: String,
    name: String,
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
            team: RefCell::new(Weak::new()),
        })
    }
    fn print(&self) {
        println!("{:?} from team {}", self, self.team.borrow().upgrade().unwrap_or(Team::default()).name);
    }
}
impl fmt::Display for Member  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(team) = self.team.borrow().upgrade() {
            write!(f, "{:?} from team {}", self, team.name)
        } else {
            write!(f, "{:?} has NO TEAM ASSIGNED", self /*default name or whatever*/)
            // or even
            // Ok(())
        }
    }
}
impl Team {
    pub fn default() -> Rc<Team> {
        Rc::new(Team {
            _id: "?".to_string(),
            name: "?".to_string(),
            members: RefCell::new(vec!()),
        })
    }
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
}
