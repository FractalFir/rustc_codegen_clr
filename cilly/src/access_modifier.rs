use serde::{Deserialize, Serialize};
/// Specifies if an item is public, or not
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub enum AccessModifer {
    /// Specifies that the given item is private
    Private,
    /// Specifies that the givem item is public
    Public,
    /// Specifies that the given item is accesible form this assembly, but the dead-code elimination should not treat it as such.
    ModulePublic,
}
