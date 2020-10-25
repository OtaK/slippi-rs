#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stage {
    id: u8,
    name: &'static str,
}

impl Stage {
    pub fn from_stage_id(stage_id: u8) -> crate::SlippiResult<Self> {
        STAGES
            .iter()
            .find(|s| s.id == stage_id)
            .map(|s| *s)
            .ok_or_else(|| crate::SlippiError::UnknownStage(stage_id))
    }
}

pub const STAGES: [Stage; 30] = [
    Stage {
        id: 2,
        name: "Fountain of Dreams",
    },
    Stage {
        id: 3,
        name: "Pokémon Stadium",
    },
    Stage {
        id: 4,
        name: "Princess Peach's Castle",
    },
    Stage {
        id: 5,
        name: "Kongo Jungle",
    },
    Stage {
        id: 6,
        name: "Brinstar",
    },
    Stage {
        id: 7,
        name: "Corneria",
    },
    Stage {
        id: 8,
        name: "Yoshi's Story",
    },
    Stage {
        id: 9,
        name: "Onett",
    },
    Stage {
        id: 10,
        name: "Mute City",
    },
    Stage {
        id: 11,
        name: "Rainbow Cruise",
    },
    Stage {
        id: 12,
        name: "Jungle Japes",
    },
    Stage {
        id: 13,
        name: "Great Bay",
    },
    Stage {
        id: 14,
        name: "Hyrule Temple",
    },
    Stage {
        id: 15,
        name: "Brinstar Depths",
    },
    Stage {
        id: 16,
        name: "Yoshi's Island",
    },
    Stage {
        id: 17,
        name: "Green Greens",
    },
    Stage {
        id: 18,
        name: "Fourside",
    },
    Stage {
        id: 19,
        name: "Mushroom Kingdom I",
    },
    Stage {
        id: 20,
        name: "Mushroom Kingdom II",
    },
    Stage {
        id: 22,
        name: "Venom",
    },
    Stage {
        id: 23,
        name: "Poké Floats",
    },
    Stage {
        id: 24,
        name: "Big Blue",
    },
    Stage {
        id: 25,
        name: "Icicle Mountain",
    },
    Stage {
        id: 26,
        name: "Icetop",
    },
    Stage {
        id: 27,
        name: "Flat Zone",
    },
    Stage {
        id: 28,
        name: "Dream Land N64",
    },
    Stage {
        id: 29,
        name: "Yoshi's Island N64",
    },
    Stage {
        id: 30,
        name: "Kongo Jungle N64",
    },
    Stage {
        id: 31,
        name: "Battlefield",
    },
    Stage {
        id: 32,
        name: "Final Destination",
    },
];

pub const STAGE_FOD: u8 = 2;
pub const STAGE_POKEMON: u8 = 3;
pub const STAGE_YOSHIS: u8 = 8;
pub const STAGE_DREAM_LAND: u8 = 28;
pub const STAGE_BATTLEFIELD: u8 = 31;
pub const STAGE_FD: u8 = 32;
