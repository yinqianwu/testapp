pub enum RelationType {
    Unknown = 0,
    Up = 1,
    Down = 2,
}

impl RelationType {
    pub fn from_u32(value: u32) -> RelationType {
        match value {
            1 => RelationType::Up,
            2 => RelationType::Down,
            _ => RelationType::Unknown,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RelationType::Up => String::from("Up"),
            RelationType::Down => String::from("Down"),
            _ => String::from("Unknown"),
        }
    }
}