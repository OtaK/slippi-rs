// TODO: Find a way to encode/decode ubjson


#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
pub enum CommunicationType {
    Handshake = 1,
    Replay = 2,
    KeepAlive = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationMessagePayload {
    cursor: Vec<u8>,
    client_token: Vec<u8>,
    pos: Vec<u8>,
    next_pos: Vec<u8>,
    data: Vec<u8>,
    nick: Option<String>,
    force_pos: bool,
    nintendont_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationMessage {
    r#type: CommunicationMessage,
    payload: CommunicationMessagePayload,
}

pub struct ConsoleCommunication {
    recv_buf: Vec<u8>,
    messages: Vec<CommunicationMessage>,
}

impl ConsoleCommunication {
    // TODO: Implement, cf. https://github.com/project-slippi/slippi-js/blob/master/src/console/communication.ts
}
