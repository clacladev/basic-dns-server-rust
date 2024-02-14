use self::header::Header;
use self::question::Question;

pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Question,
}

impl Default for Message {
    fn default() -> Self {
        Message {
            header: Header::default(),
            question: Question::default(),
        }
    }
}

#[allow(dead_code)]
impl Message {
    pub fn new() -> Self {
        Message::default()
    }
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        let header = Header::from(&bytes[0..12]);
        let question = Question::from(&bytes[12..]);
        Message { header, question }
    }
}
