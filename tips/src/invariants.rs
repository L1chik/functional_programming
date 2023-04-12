use std::convert::Infallible;
use std::ops::{Add, Mul};


fn main() {
    println!("{}", Dog::ID);

    let dog: Dog = Default::default();
    let val = 25;

    let p1 = Point::new(1, 3);
    let p2: Point<i32> = Point::new(2, 1);

    let net = NetworkTask::new();
    let db = DbTask::new();
    let tasks: Vec<&dyn Task> = vec![&net, &db];

    for task in tasks {
        task.execute();
    }

    Dog::say(&dog);
    println!("{:?}", val.say());
    println!("{:?}", p1 + p2);
}

trait Say {
    const ID: usize;

    fn say(&self) {
        println!("Woof");
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: i32,
}

impl Say for Dog {
    const ID: usize = 5;
}

impl Say for i32 {
    const ID: usize = 0;

    fn say(&self) {
        println!("my value is {}", self);
    }
}

trait FromStr {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> where Self: Sized;
}

impl FromStr for String {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string())
    }
}

impl Default for Dog {
    fn default() -> Self {
        Dog {
            name: "Bob".to_string(),
            age: 10,
        }
    }
}



#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self
    where T: Copy+ Add + Mul + Default
    {
        Point {
            x,
            y,
        }
    }
}

impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


/// Dyn polymorphism

struct NetworkTask {}
struct DbTask {}

trait Task {
    fn new() -> Self where Self: Sized;
    fn execute(&self) -> Result<(), ()> {
        Ok(())
    }
}

impl Task for NetworkTask {
    fn new() -> Self {
        NetworkTask {}
    }
}
impl Task for DbTask {
    fn new() -> Self {
        DbTask {}
    }
}