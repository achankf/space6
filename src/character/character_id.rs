use std::fmt;

use super::CharacterId;

impl CharacterId {
    pub fn new_unsafe(index: usize) -> Self {
        Self(index)
    }
}

impl From<CharacterId> for usize {
    fn from(id: CharacterId) -> Self {
        id.0
    }
}

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
