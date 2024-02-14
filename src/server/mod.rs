use crate::server::message::{answer::Answer, Message};
use anyhow::Result;
use std::net::UdpSocket;

mod message;

pub fn start_server() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        // Read request message
        let (size, source) = udp_socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, source);
        let request_message = Message::from(buf.as_slice());

        // Prepare response message
        let answer = Answer::for_question(&request_message.question);
        let response_message = request_message.response_message(answer);

        // Send response message
        let response_bytes: Vec<u8> = response_message.into();
        udp_socket.send_to(&response_bytes, source)?;
    }
}
