use std::thread;
use std::rc::Rc;

fn main() {
    // o_call();
    // t_call();
    // t_call2();

    f_call();
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