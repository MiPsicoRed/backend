use std::fmt::Display;

#[derive(Debug, Default)]
pub enum SexualOrientation {
    #[default]
    Straight,
    Gay,
    Lesbian,
    BiOrPan,
    PreferNotToSay,
}

impl Display for SexualOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SexualOrientation::Straight => write!(f, "Straight"),
            SexualOrientation::Gay => write!(f, "Gay"),
            SexualOrientation::Lesbian => write!(f, "Lesbian"),
            SexualOrientation::BiOrPan => write!(f, "Bi or Pan"),
            SexualOrientation::PreferNotToSay => write!(f, "Prefer not to say"),
        }
    }
}

impl SexualOrientation {
    pub const ALL: &'static [Self] = &[
        Self::Straight,
        Self::Gay,
        Self::Lesbian,
        Self::BiOrPan,
        Self::PreferNotToSay,
    ];

    pub fn to_id(&self) -> i32 {
        match self {
            SexualOrientation::Straight => 1,
            SexualOrientation::Gay => 2,
            SexualOrientation::Lesbian => 3,
            SexualOrientation::BiOrPan => 4,
            SexualOrientation::PreferNotToSay => 5,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(SexualOrientation::Straight),
            2 => Some(SexualOrientation::Gay),
            3 => Some(SexualOrientation::Lesbian),
            4 => Some(SexualOrientation::BiOrPan),
            5 => Some(SexualOrientation::PreferNotToSay),
            _ => None,
        }
    }
}
