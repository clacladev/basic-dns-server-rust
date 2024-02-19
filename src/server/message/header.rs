pub const HEADER_SIZE: usize = 12;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            id: 0,
            qr: 0,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

impl From<&[u8]> for Header {
    fn from(bytes: &[u8]) -> Self {
        Header {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            qr: bytes[2] >> 7,
            opcode: (bytes[2] >> 3) & 0b00001111,
            aa: (bytes[2] >> 2) & 0b00000001,
            tc: (bytes[2] >> 1) & 0b00000001,
            rd: bytes[2] & 0b00000001,
            ra: bytes[3] >> 7,
            z: (bytes[3] >> 4) & 0b00000111,
            rcode: bytes[3] & 0b00001111,
            qdcount: u16::from_be_bytes([bytes[4], bytes[5]]),
            ancount: u16::from_be_bytes([bytes[6], bytes[7]]),
            nscount: u16::from_be_bytes([bytes[8], bytes[9]]),
            arcount: u16::from_be_bytes([bytes[10], bytes[11]]),
        }
    }
}

impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        vec![
            (self.id >> 8) as u8,
            self.id as u8,
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd,
            (self.ra << 7) | (self.z << 4) | self.rcode,
            (self.qdcount >> 8) as u8,
            self.qdcount as u8,
            (self.ancount >> 8) as u8,
            self.ancount as u8,
            (self.nscount >> 8) as u8,
            self.nscount as u8,
            (self.arcount >> 8) as u8,
            self.arcount as u8,
        ]
    }
}
