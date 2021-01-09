#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[repr(u8)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Stage {
    FountainOfDreams = 2,
    PokemonStadium = 3,
    PeachsCastle = 4,
    KongoJungle = 5,
    Brinstar = 6,
    Corneria = 7,
    YoshisStory = 8,
    Onett = 9,
    MuteCity = 10,
    RainbowCruise = 11,
    JungleJapes = 12,
    GreatBay = 13,
    HyruleTemple = 14,
    BrinstarDepths = 15,
    YoshisIsland = 16,
    GreenGreens = 17,
    Fourside = 18,
    MushroomKingdom = 19,
    MushroomKingdom2 = 20,
    Venom = 22,
    PokeFloats = 23,
    BigBlue = 24,
    IcicleMountain = 25,
    Icetop = 26,
    FlatZone = 27,
    Dreamland = 28,
    YoshisIslandN64 = 29,
    KongoJungleN64 = 30,
    Battlefield = 31,
    FinalDestination = 32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageInfo {
    id: Stage,
    name: &'static str,
}

impl StageInfo {
    pub fn from_stage_id(stage_id: u8) -> crate::SlippiResult<Self> {
        STAGES
            .iter()
            .find(|s| s.id as u8 == stage_id)
            .map(|s| *s)
            .ok_or_else(|| crate::SlippiError::UnknownStage(stage_id))
    }
}

pub const STAGES: [StageInfo; 30] = [
    StageInfo {
        id: Stage::FountainOfDreams,
        name: "Fountain of Dreams",
    },
    StageInfo {
        id: Stage::PokemonStadium,
        name: "Pokémon Stadium",
    },
    StageInfo {
        id: Stage::PeachsCastle,
        name: "Princess Peach's Castle",
    },
    StageInfo {
        id: Stage::KongoJungle,
        name: "Kongo Jungle",
    },
    StageInfo {
        id: Stage::Brinstar,
        name: "Brinstar",
    },
    StageInfo {
        id: Stage::Corneria,
        name: "Corneria",
    },
    StageInfo {
        id: Stage::YoshisStory,
        name: "Yoshi's Story",
    },
    StageInfo {
        id: Stage::Onett,
        name: "Onett",
    },
    StageInfo {
        id: Stage::MuteCity,
        name: "Mute City",
    },
    StageInfo {
        id: Stage::RainbowCruise,
        name: "Rainbow Cruise",
    },
    StageInfo {
        id: Stage::JungleJapes,
        name: "Jungle Japes",
    },
    StageInfo {
        id: Stage::GreatBay,
        name: "Great Bay",
    },
    StageInfo {
        id: Stage::HyruleTemple,
        name: "Hyrule Temple",
    },
    StageInfo {
        id: Stage::BrinstarDepths,
        name: "Brinstar Depths",
    },
    StageInfo {
        id: Stage::YoshisIsland,
        name: "Yoshi's Island",
    },
    StageInfo {
        id: Stage::GreenGreens,
        name: "Green Greens",
    },
    StageInfo {
        id: Stage::Fourside,
        name: "Fourside",
    },
    StageInfo {
        id: Stage::MushroomKingdom,
        name: "Mushroom Kingdom I",
    },
    StageInfo {
        id: Stage::MushroomKingdom2,
        name: "Mushroom Kingdom II",
    },
    StageInfo {
        id: Stage::Venom,
        name: "Venom",
    },
    StageInfo {
        id: Stage::PokeFloats,
        name: "Poké Floats",
    },
    StageInfo {
        id: Stage::BigBlue,
        name: "Big Blue",
    },
    StageInfo {
        id: Stage::IcicleMountain,
        name: "Icicle Mountain",
    },
    StageInfo {
        id: Stage::Icetop,
        name: "Icetop",
    },
    StageInfo {
        id: Stage::FlatZone,
        name: "Flat Zone",
    },
    StageInfo {
        id: Stage::Dreamland,
        name: "Dream Land N64",
    },
    StageInfo {
        id: Stage::YoshisIslandN64,
        name: "Yoshi's Island N64",
    },
    StageInfo {
        id: Stage::KongoJungleN64,
        name: "Kongo Jungle N64",
    },
    StageInfo {
        id: Stage::Battlefield,
        name: "Battlefield",
    },
    StageInfo {
        id: Stage::FinalDestination,
        name: "Final Destination",
    },
];
