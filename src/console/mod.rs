pub mod communication;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
#[strum(serialize_all = "camelCase")]
pub enum ConnectionEvent {
    Connect,
    Message,
    Handshake,
    StatusChange,
    Data,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
pub enum ConnectionStatus {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    ReconnectWait = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u16)]
pub enum Ports {
    Default = 51441,
    Legacy = 666,
    RelayStart = 53741,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionDetails {
    console_nick: String,
    game_data_cursor: u8,
    version: String,
    client_token: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionSettings {
    ip_address: String,
    port: u16,
}

pub trait Connection {
    fn get_status(&self) -> &ConnectionStatus;
    fn get_settings(&self) -> &ConnectionSettings;
    fn get_details(&self) -> &ConnectionDetails;
    fn connect(&self, ip: String, port: u16) -> crate::SlippiResult<()>;
    fn disconnect(&self) -> crate::SlippiResult<()>;
}
