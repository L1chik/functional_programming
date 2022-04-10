use std::net::UdpSocket;

pub fn main() {
    let udp_socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Unable to bind to port");

    udp_socket.connect("127.0.0.1:3999")
        .expect("Couldn't connect to UDP server");
    println!("Socket peer addr is {:?}", udp_socket.peer_addr());

    udp_socket.send("Hello world, send()".as_bytes())
        .expect("Amogus");
}