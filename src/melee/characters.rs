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
    id: u8,
    name: &'static str,
    short_name: &'static str,
    colors: &'static [CharacterColor],
}

impl CharacterInfo {
    pub fn from_character_id(id: u8) -> crate::SlippiResult<Self> {
        EXTERNAL_CHARACTERS
            .iter()
            .find(|c| c.id == id)
            .map(Clone::clone)
            .ok_or_else(|| crate::SlippiError::UnknownCharacter(id))
    }
}

pub const EXTERNAL_CHARACTERS: [CharacterInfo; 26] = [
    CharacterInfo {
        id: 0,
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
        id: 1,
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
        id: 2,
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
        id: 3,
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
        id: 4,
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
        id: 5,
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
        id: 6,
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
        id: 7,
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
        id: 8,
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
        id: 9,
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
        id: 10,
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
        id: 11,
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
        id: 12,
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
        id: 13,
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
        id: 14,
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
        id: 15,
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
        id: 16,
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
        id: 17,
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
        id: 18,
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
        id: 19,
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
        id: 20,
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
        id: 21,
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
        id: 22,
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
        id: 23,
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
        id: 24,
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
        id: 25,
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
