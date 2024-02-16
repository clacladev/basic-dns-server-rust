use super::header::HEADER_SIZE;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Question {
    pub qname: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
    pub labels: Vec<String>,
}

impl Question {
    fn get_labels(bytes: &[u8]) -> (usize, Vec<String>) {
        let mut labels_tree: BTreeMap<usize, String> = BTreeMap::new();
        let mut offset = 0;

        loop {
            let label_length = bytes[offset] as usize;

            let is_null_byte = label_length == 0;
            if is_null_byte {
                break; // End of questions
            }

            // Just the initial two bits are used to indicate the length of the label
            let label_type = label_length & 0b11000000;

            let is_label_pointer = label_type == 0b11000000;
            if is_label_pointer {
                // Get the pointed label
                let offset_pointer =
                    u16::from_be_bytes([bytes[offset] & 0b00111111, bytes[offset + 1]]) as usize;
                let label = labels_tree.get(&offset_pointer).unwrap().into();

                // Add the label to the tree
                labels_tree.insert(offset + HEADER_SIZE, label);
                offset += 2;
            } else {
                // Add the label to the tree
                let label =
                    String::from_utf8(bytes[(offset + 1)..(offset + label_length + 1)].to_vec())
                        .unwrap();
                labels_tree.insert(offset + HEADER_SIZE, label);
                offset += label_length + 1;
            }
        }

        // Stitch labels together from hashmap
        let mut complete_label = String::new();
        for (_, label) in labels_tree {
            complete_label.push_str(&label);
            complete_label.push_str(".");
        }

        (offset, vec![complete_label])
    }
}

impl From<&[u8]> for Question {
    fn from(bytes: &[u8]) -> Self {
        let (offset, labels) = Question::get_labels(bytes);
        let qname = bytes[..offset].to_vec();
        let bytes = &bytes[(offset + 1)..];

        Question {
            qname,
            qtype: u16::from_be_bytes([bytes[0], bytes[1]]),
            qclass: u16::from_be_bytes([bytes[2], bytes[3]]),
            labels,
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
