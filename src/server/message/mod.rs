use self::answer::Answer;
use self::header::{Header, HEADER_SIZE};
use self::question::Question;

pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        let header = Header::from(&bytes[0..HEADER_SIZE]);
        let mut initial_offset = HEADER_SIZE;

        let (questions, questions_offset) =
            Question::from_bytes(&bytes[initial_offset..], header.qdcount, initial_offset);
        initial_offset += questions_offset;

        let (answers, _) =
            Answer::from_bytes(&bytes[initial_offset..], header.ancount, initial_offset);

        Message {
            header,
            questions,
            answers,
        }
    }
}

impl Message {
    pub fn response_message(&self, answers: Vec<Answer>) -> Self {
        let rcode: u8 = match self.header.opcode {
            0 => 0,
            _ => 4,
        };
        let response_header = Header {
            qr: 1,
            ancount: answers.len() as u16,
            rcode,
            ..self.header
        };

        Message {
            header: response_header,
            questions: self
                .questions
                .iter()
                .map(Question::uncompressed_question)
                .collect(),
            answers: answers,
        }
    }
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        let header_bytes: Vec<u8> = self.header.into();

        let questions_bytes: Vec<u8> = self
            .questions
            .iter()
            .map(|q| Into::<Vec<u8>>::into(q.clone()))
            .flatten()
            .collect();

        let answers_bytes: Vec<u8> = self
            .answers
            .iter()
            .map(|a| Into::<Vec<u8>>::into(a.clone()))
            .flatten()
            .collect();

        [header_bytes, questions_bytes, answers_bytes].concat()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const MESSAGE_BYTES: &[u8] = &[
        144, 155, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115,
        115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3,
        100, 101, 102, 192, 16, 0, 1, 0, 1,
    ];

    #[test]
    fn test_when_two_questions_with_compression_then_they_are_decompressed_correctly() {
        // Given
        // When
        let request_message = Message::from(MESSAGE_BYTES);
        // Then
        assert_eq!(request_message.header.qdcount, 2);
        assert_eq!(request_message.questions.len(), 2);
        assert_eq!(
            request_message.questions[0].qname,
            vec![
                3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110,
                110, 97, 109, 101, 3, 99, 111, 109
            ]
        );
        assert_eq!(
            request_message.questions[0].label,
            "abc.longassdomainname.com.".to_string()
        );
        assert_eq!(
            request_message.questions[1].qname,
            vec![3, 100, 101, 102, 192, 16]
        );
        assert_eq!(
            request_message.questions[1].label,
            "def.longassdomainname.com.".to_string()
        );
    }

    #[test]
    fn test_when_request_with_compressed_questions_then_response_questions_are_uncompressed() {
        // Given
        let request_message = Message::from(MESSAGE_BYTES);
        // When
        let uncompressed_question = request_message.questions[0].uncompressed_question();
        // Then
        assert_eq!(
            uncompressed_question.qname,
            vec![
                3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110,
                110, 97, 109, 101, 3, 99, 111, 109
            ]
        );
        // When
        let uncompressed_question = request_message.questions[1].uncompressed_question();
        // Then
        assert_eq!(
            uncompressed_question.qname,
            vec![
                3, 100, 101, 102, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105,
                110, 110, 97, 109, 101, 3, 99, 111, 109
            ]
        );
    }

    #[test]
    fn test_when_response_answer_questions_are_uncompressed() {
        // Given
        let request_message = Message::from(MESSAGE_BYTES);
        // When
        let answer = Answer::for_question(&request_message.questions[0]);
        // Then
        assert_eq!(
            answer.name,
            vec![
                3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110,
                110, 97, 109, 101, 3, 99, 111, 109
            ]
        );
        // When
        let answer = Answer::for_question(&request_message.questions[1]);
        // Then
        assert_eq!(
            answer.name,
            vec![
                3, 100, 101, 102, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105,
                110, 110, 97, 109, 101, 3, 99, 111, 109
            ]
        );
    }
}
