use std::cell::{Cell, RefCell};
use std::cmp::Ordering::Equal;
use std::rc::{Rc, Weak};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Boss,
    Writer,
    Blogger,
    Developer,
    Cyclist
}
#[derive(Debug)]
struct Person {
    _id: String,
    name: String,
    active: Cell<bool>,
    age: Cell<i32>,
    job: Cell<Job>,
    job_history: RefCell<Vec<Job>>,
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
    pub fn new(name: &str, job: Job, age: i32, deputy: Option<Rc<Person>>) -> Rc<Person> {
        Rc::new( Person {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            active: Cell::new(true),
            job: Cell::new(job),
            age: Cell::new(age),
            job_history: RefCell::new(vec!()),
            deputy: RefCell::new(deputy),
            team: RefCell::new(Weak::new()),
        })
    }
    fn set_job(&self, job: Job) {
        self.job_history.borrow_mut().push(self.job.get());
        self.job.set(job);
    }
    fn set_age(&self, age: i32) {
        self.age.set(age);
    }
    fn set_deputy(&self, deputy: Rc<Person>) {
        *self.deputy.borrow_mut() = Option::Some(deputy);
    }
    fn is_deputy(&self) -> bool {
        if let Some(team) = self.team.borrow().upgrade() {
            if let Some(_) = team.members.borrow().iter().find(|m|
                match m.deputy.borrow().as_ref() {
                    Some(deputy) => deputy._id == self._id,
                    None => false,
                }
            ) {
                return true;
            }
        }
        false
    }
    fn has_job(self: &Rc<Self>, job: Job) -> Option<Rc<Person>> {
        if self.job.get() == job {
            Some(Rc::clone(self))
        }
        else {
            None
        }
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
        if let Some(_) = self.members.borrow().iter().find(|m| m._id == member._id) {
            //already team member -> exit and don't add again
            return
        }
        *member.team.borrow_mut() = Rc::downgrade(self);
        self.members.borrow_mut().push(member);
    }
    fn find_members_by_job(&self, job: Job) -> Vec<Rc<Person>> {
        return self.members.borrow().iter().filter_map(|m| m.has_job(job)).collect::<Vec<Rc<Person>>>();
    }
    fn find_member_by_name(&self, name: &str) -> Option<Rc<Person>> {
        if let Some(c) = self.members.borrow().iter().find(|m|
            m.name.cmp(&name.to_string()) == Equal && m.active.get() == true
        ) {
            Some(Rc::clone(c))
        }
        else {
            Option::None
        }
    }
    fn remove_member_by_name(&self, name: &str) -> bool {
        if self.deactivate_member_by_name(name) {
            self.members.borrow_mut().retain(|m| m.name.cmp(&name.to_string()) != Equal);
            return true;
        }
        false
    }
    fn deactivate_member_by_name(&self, name: &str) -> bool {
        if let Some(c) = self.find_member_by_name(name) {
            if !c.is_deputy() {
                c.active.set(false);
                return true;
            }
        }
        false
    }
}
impl fmt::Display for Team  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let d = Person::new ("mccarthy",Job::Boss, 60, None);
    let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
    let d1 = Person::new ("bilbo",Job::Boss, 26, None);
    let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
    let team = Team::new("buddies");
    team.add_member(Rc::clone(&d1));
    team.add_member(Rc::clone(&p1));
    team.add_member(Rc::clone(&d));
    team.add_member(Rc::clone(&p));
    p.set_job(Job::Developer);
    p.set_age(44);
    p.set_job(Job::Cyclist);
    println!("{}", d);
    println!("{}", p);
    println!();
    println!("{}", team);
    println!();
    team.deactivate_member_by_name("mccarthy");
    if let Some(c) = team.find_member_by_name("mccarthy") {
        println!("Found {}", c);
    }
    println!();
    println!("{}", team);
    let x = team.find_members_by_job(Job::Cyclist);
    println!("Found {:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_member() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let t = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("employee");
        assert!(x.is_some());
        assert_eq!(x.unwrap().team.borrow().upgrade().unwrap()._id, team._id);
        let y = team.find_member_by_name("mccarthy");
        assert!(y.is_some());
        assert_eq!(y.unwrap().is_deputy(), true);
        let z = team.find_member_by_name("frodo");
        assert!(z.is_some());
        assert_eq!(z.unwrap().has_job(Job::Blogger).unwrap().name, "frodo");
    }
    #[test]
    fn test_find_member_by_name() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        let x = team.find_member_by_name("mccarthy");
        assert!(x.is_some());
        assert_eq!(x.unwrap().name, "mccarthy");
    }
    #[test]
    fn test_find_members_by_job() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        p.set_job(Job::Developer);
        p.set_job(Job::Cyclist);
        let x = team.find_members_by_job(Job::Cyclist);
        assert_eq!(x.len(), 1);
        assert_eq!(x[0].name, "employee");
    }
    #[test]
    fn test_deactivate_by_name_positive_test() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.deactivate_member_by_name("employee");
        let x = team.find_member_by_name("employee");
        assert!(x.is_none());
    }
    #[test]
    fn test_deactivate_by_name_negative_test() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.deactivate_member_by_name("mccarthy");
        let x = team.find_member_by_name("mccarthy");
        assert!(x.is_some());
    }
    #[test]
    fn test_remove_by_name_positive_test() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.remove_member_by_name("employee");
        let x = team.find_member_by_name("employee");
        assert!(x.is_none());
    }
    #[test]
    fn test_remove_by_name_negative_test() {
        let d = Person::new ("mccarthy",Job::Boss, 60, None);
        let p = Person::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Person::new ("bilbo",Job::Boss, 26, None);
        let p1 = Person::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d1));
        team.add_member(Rc::clone(&p1));
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.remove_member_by_name("mccarthy");
        let x = team.find_member_by_name("mccarthy");
        assert!(x.is_some());
    }
}