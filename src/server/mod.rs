use crate::server::message::{header::Header, Message};
use anyhow::Result;
use std::net::UdpSocket;

mod message;

pub fn start_server() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        let (size, source) = udp_socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, source);

        let message = Message::from(buf.as_slice());
        // println!("{:?}", message);

        let response_header = Header {
            qr: 1,
            qdcount: 1,
            ..message.header
        };
        let response_header_bytes: Vec<u8> = response_header.into();
        let response_question_bytes: Vec<u8> = message.question.into();

        let response: Vec<u8> = [response_header_bytes, response_question_bytes].concat();
        udp_socket.send_to(&response, source)?;
    }
}
