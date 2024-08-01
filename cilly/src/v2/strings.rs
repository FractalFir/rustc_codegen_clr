use serde::{Deserialize, Serialize};

use super::bimap::{BiMapIndex, IntoBiMapIndex};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct StringIdx(BiMapIndex);
impl IntoBiMapIndex for StringIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
