use std::fmt;
use std::fmt::{Formatter, write};
use std::hash::Hash;
use futures::executor::block_on;
use crate::Poll::Pending;


async fn say() {
    println!("hi")
}

#[derive(Debug)]
struct Song {
    title: String,
}

async fn learn_song() -> Song {
    println!("i learned the song Radio");

    Song {
        title: "Radio".to_string(),
    }
}
async fn sing_song(song: Song) {
    println!("lalala {}", song)
}

async fn dance() {
    println!("dancing cool dance")
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

async fn a_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}


// SIMPLE FUTURE TRAIT
enum Poll<T> {
    Ready(T),
    Pending,
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

struct Socket;
impl Socket {
    fn has_data(&self) -> bool {
        true
    }

    fn read_buf(&self) -> Vec<u8> {
        vec![]
    }

    fn set_readable_callback(&self, _wake: fn()) {}
}

// READ FROM SOCKET
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);

            Poll::Pending
        }
    }
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}


fn main() {
    // println!("hi main");
    //
    // let future = say();

    // block_on(a_main());

    let bar = [1, 2, 3];
    let foos = bar.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();

    let foos: Vec<_> = bar.iter()
        .map(|x| format!("{}", x))
        .collect();

    // print_type();
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

