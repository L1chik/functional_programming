mod udp_s;
mod udp_c;

use std::net;


fn main() {
    let v4 = net::Ipv4Addr::new(192, 168, 10, 10);
    let v4_2 = net::Ipv4Addr::LOCALHOST;
    println!("is loopback: {}", v4.is_loopback()); // true if local
    println!("is loopback: {}", v4_2.is_loopback());
}
