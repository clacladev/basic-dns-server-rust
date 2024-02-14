use super::question::Question;

#[derive(Debug)]
pub struct Answer {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl Answer {
    pub fn for_question(question: &Question) -> Self {
        Answer {
            name: question.qname.clone(),
            qtype: 1,
            qclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![93, 184, 216, 34], // random IP address
        }
    }
}

impl Into<Vec<u8>> for Answer {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.name);
        bytes.push(0);
        bytes.extend(self.qtype.to_be_bytes());
        bytes.extend(self.qclass.to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.rdlength.to_be_bytes());
        bytes.extend(self.rdata);
        bytes
    }
}
