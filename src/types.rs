#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    MessageSizes = 0x35,
    GameStart = 0x36,
    PreFrameUpdate = 0x37,
    PostFrameUpdate = 0x38,
    GameEnd = 0x39,
    ItemUpdate = 0x3B,
    FrameBookend = 0x3C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    player_index: u8,
    port: u8,
    character_id: Option<u8>,
    character_color: Option<u8>,
    start_stocks: Option<u8>,
    r#type: Option<u8>,
    team_id: Option<u8>,
    controller_fix: Option<&'static str>,
    nametag: Option<&'static str>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum GameMode {
    Vs = 0x02,
    Online = 0x08,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
pub enum Frames {
    First = -123,
    FirstPlayable = -39,
}
