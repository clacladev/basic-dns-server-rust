use crate::server::message::Message;
use anyhow::Result;
use std::net::UdpSocket;

mod message;

pub fn start_server() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        let (size, source) = udp_socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, source);

        let request_message = Message::from(buf.as_slice());
        let response_message = Message::response_message_for(&request_message);
        let response_bytes: Vec<u8> = response_message.into();
        udp_socket.send_to(&response_bytes, source)?;
    }
}
