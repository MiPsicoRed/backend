use std::fmt::Display;

#[derive(Debug, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
    Other,
    PreferNotToSay,
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
            Gender::PreferNotToSay => write!(f, "Prefer not to say"),
        }
    }
}

impl Gender {
    pub const ALL: &'static [Self] = &[Self::Male, Self::Female, Self::Other, Self::PreferNotToSay];

    pub fn to_id(&self) -> i32 {
        match self {
            Gender::Male => 1,
            Gender::Female => 2,
            Gender::Other => 3,
            Gender::PreferNotToSay => 4,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Gender::Male),
            2 => Some(Gender::Female),
            3 => Some(Gender::Other),
            4 => Some(Gender::PreferNotToSay),
            _ => None,
        }
    }
}
