use super::message::{answer::Answer, header::Header, question::Question};
use crate::{cli_params::CliParam, server::message::Message};
use anyhow::Result;
use rand::Rng;
use std::net::UdpSocket;

pub enum Resolver {
    Default,
    Custom(String),
}

impl From<&[CliParam]> for Resolver {
    fn from(params: &[CliParam]) -> Self {
        let resolver_param = params
            .iter()
            .find(|param| matches!(param, CliParam::Resolver(_)));
        match resolver_param {
            Some(CliParam::Resolver(value)) => Resolver::Custom(value.clone()),
            None => Resolver::Default,
        }
    }
}

pub fn resolve_questions(questions: &[Question], resolver: &Resolver) -> Result<Vec<Answer>> {
    match resolver {
        Resolver::Default => default_resolver(questions),
        Resolver::Custom(address_port) => custom_resolver(questions, address_port),
    }
}

fn default_resolver(questions: &[Question]) -> Result<Vec<Answer>> {
    Ok(questions.iter().map(Answer::for_question).collect())
}

fn custom_resolver(questions: &[Question], address_port: &String) -> Result<Vec<Answer>> {
    let udp_socket = UdpSocket::bind("0.0.0.0:0")?;
    udp_socket.connect(address_port)?;

    let mut thread_rand = rand::thread_rng();
    let mut answers = vec![];

    for question in questions {
        // Create a request
        let header = Header {
            id: thread_rand.gen(),
            qdcount: 1,
            ..Header::default()
        };
        let request_message = Message {
            header,
            questions: vec![question.uncompressed_question()],
            answers: vec![],
        };

        // Send request
        let request_bytes: Vec<u8> = request_message.into();
        udp_socket.send(&request_bytes)?;

        let mut buffer = [0; 512];
        let (size, _) = udp_socket.recv_from(&mut buffer)?;
        let bytes = &buffer[..size];
        let response_message = Message::from(bytes);
        answers.extend(response_message.answers);
    }

    Ok(answers)
}
