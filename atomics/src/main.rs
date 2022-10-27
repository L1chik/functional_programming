use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Condvar};
use std::cell::{Cell, RefCell };
use std::marker::PhantomData;
use std::time::Duration;
use std::collections::VecDeque;

fn main() {
    // o_call();
    // t_call();
    // t_call2();

    // f_call();
    // f_call2();

    // f3(&5, &mut 2);
    // f4();
    // f5(&Cell::new(4), &Cell::new(5));
    // f6(&Cell::new(vec![2, 3]));

    // c();
    // c2();
    c4();

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
    println!("{a:?}, {b:?}, {before:?}, {after:?} \n\n");

    if before != after {
        // X(); some function
    }
}

fn f6(a: &Cell<Vec<i32>>) {
    let mut v2 = a.take(); // Replaces the contents of the Cell with an empty Vec
    // dbg!(a);
    v2.push(1);
    a.set(v2);
}

fn f7(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

struct Y {
    p: *mut i32,
}

unsafe impl Send for Y {}
unsafe impl Sync for Y {}

fn c() {
    // RwwLock is a concurrent version of a RefCell
    let a = Rc::new(123);

    // thread::spawn(move || {
    //     dbg!(a);
    // });
}

fn c2() {
    // Mutex is mutual exclusion
    let n = Mutex::new(0);

    // Not parallel
    // thread::scope(|s| {
    //     for _ in 0..10 {
    //         s.spawn(|| {
    //             let mut guard = n.lock().unwrap();
    //
    //             for _ in 0..100 {
    //                 *guard += 1;
    //                 println!("{guard}");
    //             }
    //
    //             thread::sleep(Duration::from_secs(1));
    //         });
    //     }
    // });

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();

                for _ in 0..100 {
                    *guard += 1;
                    println!("{guard}");
                }

                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn c3() {
    let list = Mutex::new(Vec::from([1, 2, 3]));

    list.lock().unwrap().push(1); //сразу лочим, пушим и открываем мьютекс снова

    // Защите не дропается до конца if let {}
    if let Some(item) = list.lock().unwrap().pop() {
        // process_item(item);
    }

    // Быстрее будет использовать обычный else if
    if list.lock().unwrap().pop() == Some(1) {
        // do_something();
    }
    // Потому что if всегда оперирует boolean

    // Альтернатива для if let
    let item = list.lock().unwrap().pop();

    if let Some(item) = item {
        // process_item(item)
    }
}

fn c4() {
    let queue =  Mutex::new(VecDeque::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();

            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
        }

        t.thread().unpark();

        thread::sleep(Duration::from_secs(1));
    })
}

fn c5() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };

                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();

            thread::sleep(Duration::from_secs(1));
        }
    });
}