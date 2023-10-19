use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::fmt;
use uuid::Uuid;
use crate::team::Team;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Job {
    Boss,
    Writer,
    Blogger,
    Developer,
    Cyclist
}
#[derive(Debug)]
pub struct Member {
    _id: String,
    pub name: String,
    pub active: Cell<bool>,
    pub age: Cell<i32>,
    pub job: Cell<Job>,
    pub job_history: RefCell<Vec<Job>>,
    pub deputy: RefCell<Option<Rc<Member>>>,
    pub team: RefCell<Weak<Team>>,
}
impl Member {
    pub fn new(name: &str, job: Job, age: i32, deputy: Option<Rc<Member>>) -> Rc<Member> {
        Rc::new( Member {
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
    pub fn get_id(&self) -> &str {
        &self._id
    }
    pub fn set_job(&self, job: Job) {
        self.job_history.borrow_mut().push(self.job.get());
        self.job.set(job);
    }
    pub fn set_age(&self, age: i32) {
        self.age.set(age);
    }
    pub fn set_deputy(&self, deputy: Rc<Member>) {
        *self.deputy.borrow_mut() = Option::Some(deputy);
    }
    pub fn is_deputy(&self) -> bool {
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
    pub fn has_job(self: &Rc<Self>, job: Job) -> Option<Rc<Member>> {
        if self.job.get() == job {
            Some(Rc::clone(self))
        }
        else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deputy_chaining() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("donovan",Job::Boss, 33, Some(Rc::clone(&d)));
        let t = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&p)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("frodo");
        assert!(x.is_some());
        let binding = x.unwrap();
        let binding = binding.deputy.borrow();
        let deputy = binding.as_ref().unwrap();
        assert_eq!(deputy.name, p.name);
        assert_eq!(deputy.deputy.borrow().as_ref().unwrap().name, d.name);
    }
    #[test]
    fn test_is_deputy() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let t = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("mccarthy");
        assert!(x.is_some());
        assert_eq!(x.unwrap().is_deputy(), true);
        let y = team.find_member_by_name("frodo");
        assert!(y.is_some());
        assert_eq!(y.unwrap().is_deputy(), false);
    }
    #[test]
    fn test_set_deputy() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("donovan",Job::Boss, 33, None);
        let t = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("frodo");
        assert!(x.is_some());
        assert_eq!(x.unwrap().deputy.borrow().as_ref().unwrap()._id, d._id);
        t.set_deputy(Rc::clone(&p));
        let y = team.find_member_by_name("frodo");
        assert!(y.is_some());
        assert_eq!(y.unwrap().deputy.borrow().as_ref().unwrap()._id, p._id);
    }
    #[test]
    fn test_has_job() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let t = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("mccarthy");
        assert!(x.is_some());
        assert_eq!(x.unwrap().has_job(Job::Boss).unwrap().job.get(), Job::Boss);
        let y = team.find_member_by_name("frodo");
        assert!(y.is_some());
        assert_eq!(y.unwrap().has_job(Job::Blogger).unwrap().job.get(), Job::Blogger);
    }
    #[test]
    fn test_set_job() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        let x = team.find_member_by_name("employee");
        assert!(x.is_some());
        assert_eq!(x.unwrap().has_job(Job::Writer).unwrap().job.get(), Job::Writer);
        p.set_job(Job::Blogger);
        let y = team.find_member_by_name("employee");
        assert!(y.is_some());
        assert_eq!(y.unwrap().has_job(Job::Blogger).unwrap().job.get(), Job::Blogger);
        let z = team.find_member_by_name("employee");
        assert!(z.is_some());
        let member = z.unwrap();
        let job_history = member.job_history.borrow();
        assert_eq!(job_history.len(), 1);
        assert_eq!(job_history[0], Job::Writer);
    }
}