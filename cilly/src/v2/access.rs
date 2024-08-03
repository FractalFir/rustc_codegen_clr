use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Access {
    Extern,
    Public,
    Private,
}

impl Access {
    /// Returns `true` if the access is [`Extern`].
    ///
    /// [`Extern`]: Access::Extern
    #[must_use]
    pub fn is_extern(&self) -> bool {
        matches!(self, Self::Extern)
    }
}
