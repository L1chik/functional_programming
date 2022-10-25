use std::thread;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::Cell;

fn main() {
    // o_call();
    // t_call();
    // t_call2();

    // f_call();
    // f_call2();

    // f3(&5, &mut 2);
    // f4();
    f5(&Cell::new(4), &Cell::new(5));
}


fn o_call() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();

        // println!("{sum}, {len}");

        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}

fn t_call() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("lenght: {}", numbers.len());
        });
        s.spawn(|| {
            for i in &numbers {
                println!("{i}");
            }
        });
    });
}

fn t_call2() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    thread::scope(|s| {
        s.spawn(move || dbg!(x));
        s.spawn(move || dbg!(x));
    });
}

fn f_call() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
    println!("{a:?}, {b:?}");

}

fn f_call2() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    // thread::scope(|s| {
    //     s.spawn(move || dbg!(a));
    //     s.spawn(move || dbg!(b));
    // });

    // Naming example
    thread::spawn({
        let a = a.clone();

        move || {
            dbg!(a);
        }
    }).join().unwrap();
    thread::spawn({
        let a = a.clone();

        move || {
            dbg!(a);
        }
    }).join().unwrap();

    dbg!(a);
}

fn f3(a: &i32, b: &mut i32) {
    let before = *a;
    println!("{a}, {b}, {before}");
    *b += 1;
    println!("{a}, {b}, {before}");

    let after = *a;
    println!("{a}, {b}, {before}, {after}");

    if before != after {
        //x();
    }
}

fn f4() {
    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(2) };

    dbg!(b);
}

fn f5(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    println!("{a:?}, {b:?}, {before:?}");

    b.set(b.get() + 1);
    println!("{a:?}, {b:?}, {before:?}");

    let after = a.get();
    println!("{a:?}, {b:?}, {before:?}, {after:?}");

    if before != after {
        // X(); some function
    }
}