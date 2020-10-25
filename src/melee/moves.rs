#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Move {
    id: u8,
    name: &'static str,
    short_name: &'static str,
}

impl Move {
    pub fn from_move_id(id: u8) -> crate::SlippiResult<Self> {
        MOVES
            .iter()
            .find(|m| m.id == id)
            .map(|s| *s)
            .ok_or_else(|| crate::SlippiError::UnknownMove(id))
    }
}

pub const MOVES: [Move; 30] = [
    Move {
        id: 1,
        name: "Miscellaneous",
        short_name: "misc",
    },
    Move {
        id: 2,
        name: "Jab",
        short_name: "jab",
    },
    Move {
        id: 3,
        name: "Jab",
        short_name: "jab",
    },
    Move {
        id: 4,
        name: "Jab",
        short_name: "jab",
    },
    Move {
        id: 5,
        name: "Rapid Jabs",
        short_name: "rapid-jabs",
    },
    Move {
        id: 6,
        name: "Dash Attack",
        short_name: "dash",
    },
    Move {
        id: 7,
        name: "Forward Tilt",
        short_name: "ftilt",
    },
    Move {
        id: 8,
        name: "Up Tilt",
        short_name: "utilt",
    },
    Move {
        id: 9,
        name: "Down Tilt",
        short_name: "dtilt",
    },
    Move {
        id: 10,
        name: "Forward Smash",
        short_name: "fsmash",
    },
    Move {
        id: 11,
        name: "Up Smash",
        short_name: "usmash",
    },
    Move {
        id: 12,
        name: "Down Smash",
        short_name: "dsmash",
    },
    Move {
        id: 13,
        name: "Neutral Air",
        short_name: "nair",
    },
    Move {
        id: 14,
        name: "Forward Air",
        short_name: "fair",
    },
    Move {
        id: 15,
        name: "Back Air",
        short_name: "bair",
    },
    Move {
        id: 16,
        name: "Up Air",
        short_name: "uair",
    },
    Move {
        id: 17,
        name: "Down Air",
        short_name: "dair",
    },
    Move {
        id: 18,
        name: "Neutral B",
        short_name: "neutral-b",
    },
    Move {
        id: 19,
        name: "Side B",
        short_name: "side-b",
    },
    Move {
        id: 20,
        name: "Up B",
        short_name: "up-b",
    },
    Move {
        id: 21,
        name: "Down B",
        short_name: "down-b",
    },
    Move {
        id: 50,
        name: "Getup Attack",
        short_name: "getup",
    },
    Move {
        id: 51,
        name: "Getup Attack (Slow)",
        short_name: "getup-slow",
    },
    Move {
        id: 52,
        name: "Grab Pummel",
        short_name: "pummel",
    },
    Move {
        id: 53,
        name: "Forward Throw",
        short_name: "fthrow",
    },
    Move {
        id: 54,
        name: "Back Throw",
        short_name: "bthrow",
    },
    Move {
        id: 55,
        name: "Up Throw",
        short_name: "uthrow",
    },
    Move {
        id: 56,
        name: "Down Throw",
        short_name: "dthrow",
    },
    Move {
        id: 61,
        name: "Edge Attack (Slow)",
        short_name: "edge-slow",
    },
    Move {
        id: 62,
        name: "Edge Attack",
        short_name: "edge",
    },
];
