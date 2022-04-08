use std::str;
use std::net;
use std::thread;

fn server() {
    let udp_socket = net::UdpSocket::bind("127.0.0.1:3999")
        .expect("Unable to bind socket");
    let buff = [0; 1024];

    loop {

    }
}