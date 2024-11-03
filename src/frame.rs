use std::{io::Read, net::TcpStream};

use crate::ws_client::WsClientError;

#[derive(Debug)]
pub struct Frame {
    pub fin: bool,
    pub rsv1: bool,
    pub rsv2: bool,
    pub rsv3: bool,
    pub opcode: u8,
    pub mask: bool,
    pub payload_length: u64,
    pub masking_key: Option<[u8; 4]>,
    pub payload_data: Vec<u8>,
}

impl Frame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = (if self.fin { 0x80 } else { 0x00 })
            | (if self.rsv1 { 0x40 } else { 0x00 })
            | (if self.rsv2 { 0x20 } else { 0x00 })
            | (if self.rsv3 { 0x10 } else { 0x00 })
            | self.opcode;
        bytes.push(first_byte);

        let second_byte = (if self.mask { 0x80 } else { 0x00 })
            | (if self.payload_length < 126 {
                self.payload_length as u8
            } else if self.payload_length <= 0xFFFF {
                126
            } else {
                127
            });
        bytes.push(second_byte);

        if self.payload_length > 125 && self.payload_length <= 0xFFFF {
            bytes.push((self.payload_length >> 8) as u8);
            bytes.push(self.payload_length as u8);
        } else if self.payload_length > 0xFFFF {
            bytes.push((self.payload_length >> 56) as u8);
            bytes.push((self.payload_length >> 48) as u8);
            bytes.push((self.payload_length >> 40) as u8);
            bytes.push((self.payload_length >> 32) as u8);
            bytes.push((self.payload_length >> 24) as u8);
            bytes.push((self.payload_length >> 16) as u8);
            bytes.push((self.payload_length >> 8) as u8);
            bytes.push(self.payload_length as u8);
        }

        if let Some(masking_key) = self.masking_key {
            bytes.extend_from_slice(&masking_key);
        }

        bytes.extend_from_slice(&self.payload_data);

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Frame {
        let first_byte = bytes[0];
        let second_byte = bytes[1];

        let fin = first_byte & 0x80 != 0;
        let rsv1 = first_byte & 0x40 != 0;
        let rsv2 = first_byte & 0x20 != 0;
        let rsv3 = first_byte & 0x10 != 0;
        let opcode = first_byte & 0x0F;

        let mask = second_byte & 0x80 != 0;
        let payload_length = second_byte & 0x7F;

        // TODO: for now, we're assuming the payload length is less than 126
        let rest = &bytes[2..];

        Frame {
            fin,
            rsv1,
            rsv2,
            rsv3,
            opcode,
            mask,
            payload_length: payload_length as u64,
            masking_key: None,
            payload_data: rest.to_vec(),
        }
    }
}
