#[derive(Default)]
pub struct LocalUsage {
    byval_direct: usize,
    byval_indirect: usize,
    byaddres_direct: usize,
    byaddres_indirect: usize,
    assign: usize,
}
impl LocalUsage {
    pub fn is_assigned(&self) -> bool {
        self.assign != 0
    }
    pub fn is_address_taken(&self) -> bool {
        self.byaddres_direct != 0 && self.byaddres_indirect != 0
    }
    pub fn is_val_taken(&self) {
        self.byval_direct != 0 && self.byaddres_indirect != 0
    }
    pub fn is_only_indirect(&self) {
        self.byaddres_direct == 0 && self.byval_direct == 0
    }
    /// Increments the interal direct byval access counter.
    pub fn byval_direct(&mut self) {
        self.byval_direct += 1;
    }
    /// Increments the interal direct byval access counter.
    /// ```
    /// # use cilly::v2::opt::local_usage::LocalUsage;
    /// let mut access =
    /// ```
    pub fn byaddr_direct(&mut self) {
        self.byaddres_direct += 1;
    }
}
struct MethodLocalUsages(Box<[LocalUsage]>);
