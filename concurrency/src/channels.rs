use std::thread;
use std::fs;
use std::sync::mpsc;


struct Filenames {
    source: String,
    destination: String,
}

impl Drop for Filenames {
    fn drop(&mut self) {
        if thread::panicking() {
            println!("dropped due to panic");
        } else {
            println!("dropped without panic");
        }
    }
}

fn main() {
    // thread1();
    // thread2();
    // thread3();
    // thread4();
    thread5();
}

fn thread1() {
    for _ in 1..5 {
        thread::spawn(|| {
            println!("Hi from thread id {:?}",
                thread::current().id());
        });
    }
}

fn thread2() {
    let mut child_threads = Vec::new();

    for i in 1..5 {
        let builder = thread::Builder::new().name(format!("mythread{}", i));

        let handle = builder.spawn(|| {
            println!(" Hi from thread is {:?}",
            thread::current().id());
        }).unwrap();
        child_threads.push(handle);
    }

    for i in child_threads {
        i.join().unwrap();
    }
}

fn copy_file() -> thread::Result<()> {
    thread::spawn(|| {
        fs::copy("a.txt", "b.txt").expect("Error occured");
    }).join()
}

fn copy_file2(file_struct: Filenames) -> thread::Result<()> {
    thread::spawn(move || {
        fs::copy(&file_struct.source, &file_struct.destination)
            .expect("Error occurred");
    }).join()
}

fn thread3() {
    match copy_file() {
        Ok(_) => println!("Ok. copied"),
        Err(_) => println!("Error in copying file"),
    }
}

fn thread4() {
    let foo = Filenames {
        source: "a1.txt".into(),
        destination: "b.txt".into(),
    };

    match copy_file2(foo) {
        Ok(_) => println!("Copied"),
        Err(_) => println!("Error in copying file"),
    }
}

fn thread5() {
    let (transmitter1, receiver) = mpsc::channel();
    let transmitter2 = mpsc::Sender::clone(&transmitter1);

    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["one".into(), "two".into(),
                                        "three".into(), "four".into()];

        for num in num_vec {
            transmitter1.send(num).unwrap();
        }
    });

    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["Five".into(), "Six".into(),
                                        "Seven".into(), "eight".into()];

        for num in num_vec {
            transmitter2.send(num).unwrap();
        }
    });

    for received_val in receiver {
        println!("Recieved from thread: {}", received_val);
    }
}