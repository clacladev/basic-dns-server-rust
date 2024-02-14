use self::answer::Answer;
use self::header::Header;
use self::question::Question;

pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Question,
    pub answer: Option<Answer>,
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        Message {
            header: Header::from(&bytes[0..12]),
            question: Question::from(&bytes[12..]),
            answer: None,
        }
    }
}

impl Message {
    pub fn response_message(&self, answer: Answer) -> Self {
        let rcode: u8 = match self.header.opcode {
            0 => 0,
            _ => 4,
        };
        let response_header = Header {
            qr: 1,
            qdcount: 1,
            ancount: 1,
            rcode,
            ..self.header
        };
        Message {
            header: response_header,
            question: self.question.clone(),
            answer: Some(answer),
        }
    }
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        let response_header_bytes: Vec<u8> = self.header.into();
        let response_question_bytes: Vec<u8> = self.question.into();
        let answer_bytes: Vec<u8> = match self.answer {
            Some(answer) => answer.into(),
            None => vec![],
        };

        [response_header_bytes, response_question_bytes, answer_bytes].concat()
    }
}
