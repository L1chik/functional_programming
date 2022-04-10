use std::str;
use std::net;
use std::thread;

pub fn main() {
    let udp_socket = net::UdpSocket::bind("127.0.0.1:3999")
        .expect("Unable to bind to port");
    let mut buff = [0; 1024];

    loop {
        let udp_socket_new = udp_socket.try_clone()
            .expect("Unable to clone socket");

        match udp_socket_new.recv_from(&mut buff) {
            Ok((num_bytes, src_addr)) => {
                thread::spawn(move || {
                    let send_buff = &mut buff[..num_bytes];
                    println!("Received from client: {}",
                    str::from_utf8(send_buff).unwrap());

                    let response_string = format!("Received this: {}",
                    String::from_utf8_lossy(send_buff));

                    udp_socket_new.send_to(&response_string.as_bytes(), &src_addr)
                        .expect("Error in sending datagram")
                });
            }
            Err(e) => {
                println!("Error in receiving datagrams over UDP: {}", e);
            }
        }
    }
}