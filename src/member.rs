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
