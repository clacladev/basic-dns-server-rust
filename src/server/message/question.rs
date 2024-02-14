#[derive(Debug, Clone)]
pub struct Question {
    pub qname: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
}

impl From<&[u8]> for Question {
    fn from(bytes: &[u8]) -> Self {
        let qname = match bytes.iter().position(|&byte| byte == 0) {
            Some(null_index) => bytes[..null_index].to_vec(),
            None => vec![],
        };

        let bytes = &bytes[qname.len() + 1..];

        Question {
            qname,
            qtype: u16::from_be_bytes([bytes[0], bytes[1]]),
            qclass: u16::from_be_bytes([bytes[2], bytes[3]]),
        }
    }
}

impl Into<Vec<u8>> for Question {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.qname);
        bytes.push(0);
        bytes.extend(self.qtype.to_be_bytes());
        bytes.extend(self.qclass.to_be_bytes());
        bytes
    }
}
