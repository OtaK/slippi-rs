#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum DeathDirection {
    Down,
    Left,
    Right,
    Up,
}

impl DeathDirection {
    pub fn from_action_state_id(action_state: u8) -> Option<Self> {
        match action_state {
            _ if action_state > 0xA => None,
            0 => Some(Self::Down),
            1 => Some(Self::Left),
            2 => Some(Self::Right),
            _ => Some(Self::Up),
        }
    }
}
