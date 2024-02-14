use super::question::Question;

#[derive(Debug)]
pub struct Answer {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            name: String::new(),
            qtype: 0,
            qclass: 0,
            ttl: 0,
            rdlength: 0,
            rdata: Vec::new(),
        }
    }
}

impl Answer {
    pub fn for_question(_question: &Question) -> Self {
        Answer::default()
    }
}

impl Into<Vec<u8>> for Answer {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.name.as_bytes());
        bytes.push(0);
        bytes.extend(self.qtype.to_be_bytes());
        bytes.extend(self.qclass.to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.rdlength.to_be_bytes());
        bytes.extend(self.rdata);
        bytes
    }
}
