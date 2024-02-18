use super::header::HEADER_SIZE;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Question {
    pub qname: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
    pub label: String,
}

impl Question {
    pub fn from_bytes(bytes: &[u8], qdcount: u16) -> Vec<Self> {
        let mut questions = vec![];
        let mut labels_tree: BTreeMap<usize, String> = BTreeMap::new();
        let mut offset = 0;

        for _ in 0..qdcount {
            let label_start_offset = offset + HEADER_SIZE;
            let mut is_label_ending_with_pointer = false;
            loop {
                let length_byte = bytes[offset] as usize;

                let is_null_byte = length_byte == 0;
                if is_null_byte {
                    // Add end of current question token (empty string)
                    labels_tree.insert(offset + HEADER_SIZE, String::new());
                    offset += 1;
                    break;
                }

                // Just the initial two bits are used to indicate the length of the label
                let label_type = length_byte & 0b11000000;

                let is_label_pointer = label_type == 0b11000000;
                if is_label_pointer {
                    // Get the pointed label
                    let offset_pointer =
                        u16::from_be_bytes([bytes[offset] & 0b00111111, bytes[offset + 1]])
                            as usize;
                    let label = Self::get_labels_from_btree(&labels_tree, offset_pointer);
                    let label = label[..label.len() - 1].to_string(); // Remove the trailing dot

                    // Add the label to the tree
                    labels_tree.insert(offset + HEADER_SIZE, label);
                    offset += 2;

                    // Add end of current question token (empty string)
                    labels_tree.insert(offset + HEADER_SIZE, String::new());
                    is_label_ending_with_pointer = true;
                    break;
                }

                // Add the label to the tree
                let label =
                    String::from_utf8(bytes[(offset + 1)..(offset + length_byte + 1)].to_vec())
                        .unwrap();
                labels_tree.insert(offset + HEADER_SIZE, label);
                offset += length_byte + 1;
            }

            let label = Self::get_labels_from_btree(&labels_tree, label_start_offset);
            let qname = if is_label_ending_with_pointer {
                bytes[(label_start_offset - HEADER_SIZE)..offset].to_vec()
            } else {
                bytes[(label_start_offset - HEADER_SIZE)..offset - 1].to_vec()
            };
            let qtype = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let qclass = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            questions.push(Question {
                qname,
                qtype,
                qclass,
                label,
            });
        }

        return questions;
    }

    fn get_labels_from_btree(labels_tree: &BTreeMap<usize, String>, from_offset: usize) -> String {
        let mut complete_label = String::new();
        for (offset, label) in labels_tree {
            if *offset < from_offset {
                continue;
            }
            if label.is_empty() {
                break;
            }
            complete_label.push_str(label);
            complete_label.push_str(".");
        }
        complete_label
    }
}

impl Question {
    pub fn uncompressed_question(&self) -> Self {
        let label = self.label.clone();
        let qname = label
            .split(".")
            .map(|label| {
                let mut label_bytes = label.as_bytes().to_vec();
                let length = label_bytes.len() as u8;
                if length > 0 {
                    label_bytes.insert(0, length);
                }
                label_bytes
            })
            .flatten()
            .collect();

        Question {
            qname,
            qtype: self.qtype,
            qclass: self.qclass,
            label,
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
