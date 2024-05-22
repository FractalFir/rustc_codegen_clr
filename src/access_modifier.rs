use serde::{Deserialize, Serialize};
/// Specifies if an item is public, or not
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub enum AccessModifer {
    /// Specifies that the given item is private
    Private,
    /// Specifies that the givem item is public
    Public,
}
impl AccessModifer {
    /// Converts Rust visibilty type to [`AccessModifer`]
    pub fn from_visibility<T>(visibility: &rustc_middle::ty::Visibility<T>) -> Self {
        if let rustc_middle::ty::Visibility::Public = visibility {
            Self::Public
        } else {
            Self::Private
        }
    }
}
