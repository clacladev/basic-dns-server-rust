use super::question::Question;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Answer {
    pub name: Vec<u8>,
    pub atype: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
    pub label: String,
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            name: vec![],
            atype: 0,
            class: 0,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
            label: String::new(),
        }
    }
}

impl Answer {
    pub fn for_question(question: &Question) -> Self {
        Answer {
            name: question.uncompressed_question().qname,
            rdata: vec![0, 0, 0, 0], // random IP address
            label: question.label.clone(),
            ..Default::default()
        }
    }
}

impl Answer {
    pub fn from_bytes(bytes: &[u8], ancount: u16, initial_offset: usize) -> (Vec<Self>, usize) {
        let mut answers: Vec<Answer> = vec![];
        let mut labels_tree: BTreeMap<usize, String> = BTreeMap::new();
        let mut offset = 0;

        for _ in 0..ancount {
            let label_start_offset = offset + initial_offset;
            let mut is_label_ending_with_pointer = false;
            loop {
                let length_byte = bytes[offset] as usize;

                let is_null_byte = length_byte == 0;
                if is_null_byte {
                    // Add end of current question token (empty string)
                    labels_tree.insert(offset + initial_offset, String::new());
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
                    labels_tree.insert(offset + initial_offset, label);
                    offset += 2;

                    // Add end of current question token (empty string)
                    labels_tree.insert(offset + initial_offset, String::new());
                    is_label_ending_with_pointer = true;
                    break;
                }

                // Add the label to the tree
                let label =
                    String::from_utf8(bytes[(offset + 1)..(offset + length_byte + 1)].to_vec())
                        .unwrap();
                labels_tree.insert(offset + initial_offset, label);
                offset += length_byte + 1;
            }

            let label = Self::get_labels_from_btree(&labels_tree, label_start_offset);
            let name = if is_label_ending_with_pointer {
                bytes[(label_start_offset - initial_offset)..offset].to_vec()
            } else {
                bytes[(label_start_offset - initial_offset)..offset - 1].to_vec()
            };
            let atype = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let class = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            let ttl = u32::from_be_bytes([
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
            let rdlength = u16::from_be_bytes([bytes[offset + 8], bytes[offset + 9]]);
            let rdata = bytes[(offset + 10)..(offset + 10 + rdlength as usize)].to_vec();
            offset += 10 + rdlength as usize;

            answers.push(Answer {
                name,
                atype,
                class,
                ttl,
                rdlength,
                rdata,
                label,
            });
        }

        return (answers, offset);
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

impl Into<Vec<u8>> for Answer {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.name);
        bytes.push(0);
        bytes.extend(self.atype.to_be_bytes());
        bytes.extend(self.class.to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.rdlength.to_be_bytes());
        bytes.extend(self.rdata);
        bytes
    }
}
