use std::fmt::Error;
use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
use std::process::Output;
use std::rc::{Rc, Weak};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn one(args: Vec<String>) -> String {
    args
        .iter()
        .flat_map(|x| x.parse::<i32>())
        .sum::<i32>()
        .to_string()

    // println!("{sum}");
    // sum.to_string()
}

fn weak_check () {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("rc count {}", Rc::strong_count(&a));
    println!("next item {:?}", a.tail());

    let b = Rc::new(Cons(10,
                         RefCell::new(Rc::clone(&a))));

    println!("rc count after b {}", Rc::strong_count(&a));
    println!("b initial rc count {}", Rc::strong_count(&b));
    println!("b next item {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a {}", Rc::strong_count(&b));
    println!("a rc count after changing a {}", Rc::strong_count(&a));
}


fn main() {
    // let nums = vec!["1".to_string(), "ab".to_string(), "3".to_string()];
    // println!("{:?}", one(nums));

    weak_check()
}