
mod member;
mod team;
use std::rc::{Rc};
use member::{Job, Member};
use team::Team;

fn main() {
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