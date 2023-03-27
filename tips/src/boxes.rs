use crate::List::{Cons, Nil};
use std::ops::Deref;


fn main() {
    let _list = Cons(1, Box::new(Cons(1, Box::new(Cons(3, Box::new(Nil))))));

    deref_pr();
    deref_mybox();

    let a = MyBox::new(String::from("Rust"));
    // implicit deref conercion -> &MyBox<String> => &String
    hello(&a);

    //1. (*a) -> MyBox<String> => String
    //2. '&' and '[..]. take a string slice of the String
    hello(&(*a)[..]);

    dropping();
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn deref_pr() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
//    assert_eq!(5, y);
    assert_eq!(5, *y);

    let y = Box::new(x);

    // Box points to copie of value
    assert_eq!(5, x);
    assert_eq!(5, *y)
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn deref_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *(y.deref())
    assert_eq!(5, *y);
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {name}");
}

struct CustomDrop {
 data: String,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping data {}", self.data);
    }
}

fn dropping() {
    let a = CustomDrop{
        data: "first".to_string(),
    };

    {
        let b = CustomDrop {
            data: "second".to_string(),
        };
    }

    drop(a);
    println!("Pointers created")
}