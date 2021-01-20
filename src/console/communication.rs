// TODO: Find a way to encode/decode ubjson
use byteorder::{BE, ReadBytesExt as _, WriteBytesExt as _};
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
pub enum CommunicationType {
    Handshake = 1,
    Replay = 2,
    KeepAlive = 3,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CommunicationMessagePayload {
    cursor: Vec<u8>,
    client_token: Vec<u8>,
    pos: Vec<u8>,
    next_pos: Vec<u8>,
    data: Vec<u8>,
    nick: Option<String>,
    force_pos: bool,
    nintendont_version: Option<String>,
    is_realtime: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationMessage {
    r#type: CommunicationType,
    payload: CommunicationMessagePayload,
}

#[derive(Debug, Default, Clone)]
pub struct ConsoleCommunication {
    recv_buf: Cursor<Vec<u8>>,
    messages: Vec<CommunicationMessage>,
}

impl ConsoleCommunication {
    pub fn receive(&mut self, data: Vec<u8>) -> std::io::Result<()> {
        self.recv_buf.get_mut().append(&mut data);

        while self.recv_buf.get_ref().len() >= 4 {
            let msg_size = self.recv_buf.read_u32::<BE>()? as usize;
            if self.recv_buf.get_ref().len() < msg_size + 4 {
                return Ok(());
            }

            let (_, ubjson_msg) = ubjson::parse_one(&self.recv_buf.get_ref()[4..=msg_size + 4]).unwrap();
            self.messages.push(ubjson_msg); // TODO: serde impl
            self.recv_buf.get_mut().drain(..msg_size + 4);
        }

        Ok(())
    }

    pub fn buffer(&self) -> &[u8] {
        &self.recv_buf.get_ref()
    }

    pub fn consume_messages(&mut self) -> Vec<CommunicationMessage> {
        self.messages.drain(..).collect()
    }

    pub fn generate_handshake_out(buf: &[u8], token: u32, is_realtime: Option<bool>) -> Vec<u8> {
        let message = CommunicationMessage {
            r#type: CommunicationType::Handshake,
            payload: CommunicationMessagePayload {
                cursor: buf.into(),
                client_token: token.to_be_bytes().into(),
                is_realtime: Some(true),
                ..Default::default()
            }
        };
        let mut encoded = Vec::new();

        let mut ret = Vec::with_capacity(4 + encoded.len());
        ret.write_u32::<BE>(encoded.len() as u32);
        ret.append(&mut encoded);
        ret
    }
}
