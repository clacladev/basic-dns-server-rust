use self::header::Header;

pub mod header;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
}

impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        let header = Header::from(&bytes[0..12]);
        Message { header }
    }
}
