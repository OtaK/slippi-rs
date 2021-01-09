#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Character {
    CaptainFalcon = 0,
    DonkeyKong = 1,
    Fox = 2,
    GameAndWatch = 3,
    Kirby = 4,
    Bowser = 5,
    Link = 6,
    Luigi = 7,
    Mario = 8,
    Marth = 9,
    Mewtwo = 10,
    Ness = 11,
    Peach = 12,
    Pikachu = 13,
    IceClimbers = 14,
    Jigglypuff = 15,
    Samus = 16,
    Yoshi = 17,
    Zelda = 18,
    Sheik = 19,
    Falco = 20,
    YoungLink = 21,
    DrMario = 22,
    Roy = 23,
    Pichu = 24,
    Ganondorf = 25,
  }

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
#[strum(serialize_all = "title_case")]
pub enum CharacterColor {
    Default,
    Black,
    Red,
    White,
    Green,
    Blue,
    Yellow,
    Daisy,
    PartyHat,
    CowboyHat,
    Orange,
    Headband,
    Crown,
    Purple,
    Cyan,
    Pink,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterInfo {
    id: Character,
    name: &'static str,
    short_name: &'static str,
    colors: &'static [CharacterColor],
}

impl CharacterInfo {
    pub fn from_character_id(id: u8) -> crate::SlippiResult<Self> {
        EXTERNAL_CHARACTERS
            .iter()
            .find(|c| c.id as u8 == id)
            .map(Clone::clone)
            .ok_or_else(|| crate::SlippiError::UnknownCharacter(id))
    }
}

pub const EXTERNAL_CHARACTERS: [CharacterInfo; 26] = [
    CharacterInfo {
        id: Character::CaptainFalcon,
        name: "Captain Falcon",
        short_name: "Falcon",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Black,
            CharacterColor::Red,
            CharacterColor::White,
            CharacterColor::Green,
            CharacterColor::Blue,
        ],
    },
    CharacterInfo {
        id: Character::DonkeyKong,
        name: "Donkey Kong",
        short_name: "DK",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Black,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Fox,
        name: "Fox",
        short_name: "Fox",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::GameAndWatch,
        name: "Mr. Game & Watch",
        short_name: "G&W",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Kirby,
        name: "Kirby",
        short_name: "Kirby",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Yellow,
            CharacterColor::Blue,
            CharacterColor::Red,
            CharacterColor::Green,
            CharacterColor::White,
        ],
    },
    CharacterInfo {
        id: Character::Bowser,
        name: "Bowser",
        short_name: "Bowser",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Black,
        ],
    },
    CharacterInfo {
        id: Character::Link,
        name: "Link",
        short_name: "Link",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Black,
            CharacterColor::White,
        ],
    },
    CharacterInfo {
        id: Character::Luigi,
        name: "Luigi",
        short_name: "Luigi",
        colors: &[
            CharacterColor::Default,
            CharacterColor::White,
            CharacterColor::Blue,
            CharacterColor::Red,
        ],
    },
    CharacterInfo {
        id: Character::Mario,
        name: "Mario",
        short_name: "Mario",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Yellow,
            CharacterColor::Black,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Marth,
        name: "Marth",
        short_name: "Marth",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Green,
            CharacterColor::Black,
            CharacterColor::White,
        ],
    },
    CharacterInfo {
        id: Character::Mewtwo,
        name: "Mewtwo",
        short_name: "Mewtwo",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Ness,
        name: "Ness",
        short_name: "Ness",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Yellow,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Peach,
        name: "Peach",
        short_name: "Peach",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Daisy,
            CharacterColor::White,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Pikachu,
        name: "Pikachu",
        short_name: "Pikachu",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::PartyHat,
            CharacterColor::CowboyHat,
        ],
    },
    CharacterInfo {
        id: Character::IceClimbers,
        name: "Ice Climbers",
        short_name: "ICs",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Green,
            CharacterColor::Orange,
            CharacterColor::Red,
        ],
    },
    CharacterInfo {
        id: Character::Jigglypuff,
        name: "Jigglypuff",
        short_name: "Puff",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Headband,
            CharacterColor::Crown,
        ],
    },
    CharacterInfo {
        id: Character::Samus,
        name: "Samus",
        short_name: "Samus",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Pink,
            CharacterColor::Black,
            CharacterColor::Green,
            CharacterColor::Purple,
        ],
    },
    CharacterInfo {
        id: Character::Yoshi,
        name: "Yoshi",
        short_name: "Yoshi",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Yellow,
            CharacterColor::Pink,
            CharacterColor::Cyan,
        ],
    },
    CharacterInfo {
        id: Character::Zelda,
        name: "Zelda",
        short_name: "Zelda",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
            CharacterColor::White,
        ],
    },
    CharacterInfo {
        id: Character::Sheik,
        name: "Sheik",
        short_name: "Sheik",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
            CharacterColor::White,
        ],
    },
    CharacterInfo {
        id: Character::Falco,
        name: "Falco",
        short_name: "Falco",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::YoungLink,
        name: "Young Link",
        short_name: "YLink",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::White,
            CharacterColor::Black,
        ],
    },
    CharacterInfo {
        id: Character::DrMario,
        name: "Dr. Mario",
        short_name: "Doc",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
            CharacterColor::Black,
        ],
    },
    CharacterInfo {
        id: Character::Roy,
        name: "Roy",
        short_name: "Roy",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
            CharacterColor::Yellow,
        ],
    },
    CharacterInfo {
        id: Character::Pichu,
        name: "Pichu",
        short_name: "Pichu",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
        ],
    },
    CharacterInfo {
        id: Character::Ganondorf,
        name: "Ganondorf",
        short_name: "Ganon",
        colors: &[
            CharacterColor::Default,
            CharacterColor::Red,
            CharacterColor::Blue,
            CharacterColor::Green,
            CharacterColor::Purple,
        ],
    },
];
