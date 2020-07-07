
pub enum StatusType {

    Success = 0x0000,

    UnknownError = 0x0001,

    ArticleNotFoundError = 0x0100,

}

impl StatusType {
    pub fn from_u32(value: u32) -> StatusType {
        match value {
            0x0000 => StatusType::Success,
            0x0001 => StatusType::UnknownError,
            0x0100 => StatusType::ArticleNotFoundError,
            _ => StatusType::UnknownError,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            StatusType::Success => String::from("success"),
            StatusType::ArticleNotFoundError => String::from("cannot find article"),
            _ => String::from("unknown error"),
        }
    }
}

