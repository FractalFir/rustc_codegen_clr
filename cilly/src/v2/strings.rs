use super::bimap::HashWrapper;

#[derive(Hash, PartialEq, Eq, Clone, Default, Debug, Copy)]
pub struct StringIdx(u64);
impl HashWrapper for StringIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
