use std::cell::{RefCell};
use std::cmp::Ordering::Equal;
use std::rc::{Rc};
use std::fmt;
use uuid::Uuid;
use crate::member::{Job, Member};

#[derive(Debug)]
pub struct Team {
    _id: String,
    pub name: String,
    pub members: RefCell<Vec<Rc<Member>>>,
}

impl Team {
    pub fn new(name: &str) -> Rc<Team> {
        Rc::new(Team {
            _id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            members: RefCell::new(vec!()),
        })
    }
    pub fn get_id(&self) -> &str {
        &self._id
    }
    pub fn add_member(self: &Rc<Self>, member: Rc<Member>) {
        if let Some(_) = self.members.borrow().iter().find(|m| m.get_id() == member.get_id()) {
            //already team member -> exit and don't add again
            return
        }
        *member.team.borrow_mut() = Rc::downgrade(self);
        self.members.borrow_mut().push(member);
    }
    pub fn find_members_by_job(&self, job: Job) -> Vec<Rc<Member>> {
        return self.members.borrow().iter().filter_map(|m| m.has_job(job)).collect::<Vec<Rc<Member>>>();
    }
    pub fn find_member_by_name(&self, name: &str) -> Option<Rc<Member>> {
        if let Some(c) = self.members.borrow().iter().find(|m|
            m.name.cmp(&name.to_string()) == Equal && m.active.get() == true
        ) {
            Some(Rc::clone(c))
        }
        else {
            Option::None
        }
    }
    pub fn remove_member_by_name(&self, name: &str) -> bool {
        if self.deactivate_member_by_name(name) {
            self.members.borrow_mut().retain(|m| m.name.cmp(&name.to_string()) != Equal);
            return true;
        }
        false
    }
    pub fn deactivate_member_by_name(&self, name: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_member() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let t = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d)));
        let team = Team::new("buddies");
        team.add_member(Rc::clone(&d));
        team.add_member(Rc::clone(&p));
        team.add_member(Rc::clone(&t));
        let x = team.find_member_by_name("employee");
        assert!(x.is_some());
        assert_eq!(x.unwrap().team.borrow().upgrade().unwrap()._id, team.get_id()); //team._id would work as well
        let y = team.find_member_by_name("mccarthy");
        assert!(y.is_some());
        assert_eq!(y.unwrap().is_deputy(), true);
        let z = team.find_member_by_name("frodo");
        assert!(z.is_some());
        assert_eq!(z.unwrap().has_job(Job::Blogger).unwrap().job.get(), Job::Blogger);
    }
    #[test]
    fn test_find_member_by_name() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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
        assert_eq!(x[0].job_history.borrow().len(), 2);
    }
    #[test]
    fn test_deactivate_by_name_positive_test() {
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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
        let d = Member::new ("mccarthy",Job::Boss, 60, None);
        let p = Member::new ("employee",Job::Writer, 33, Some(Rc::clone(&d)));
        let d1 = Member::new ("bilbo",Job::Boss, 26, None);
        let p1 = Member::new ("frodo",Job::Blogger, 19, Some(Rc::clone(&d1)));
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