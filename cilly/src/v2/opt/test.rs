#[cfg(test)]
use crate::v2::{Assembly, BasicBlock, CILRoot};

#[test]
fn sfi_dedup() {
    let mut asm = Assembly::default();
    let file = asm.alloc_string("uwu.rs");
    let sfi_a = asm.alloc_root(CILRoot::SourceFileInfo {
        line_start: 0,
        line_len: 0,
        col_start: 0,
        col_len: 0,
        file,
    });
    let mut bb = BasicBlock::new(vec![sfi_a, sfi_a], 0, None);
    assert_eq!(bb.roots().len(), 2);
    bb.remove_duplicate_sfi(&mut asm);
    assert_eq!(bb.roots().len(), 1);
    let file = asm.alloc_string("owo.rs");
    let sfi_b = asm.alloc_root(CILRoot::SourceFileInfo {
        line_start: 0,
        line_len: 0,
        col_start: 0,
        col_len: 0,
        file,
    });
    let mut bb = BasicBlock::new(vec![sfi_a, sfi_b, sfi_a], 0, None);
    assert_eq!(bb.roots().len(), 3);
    bb.remove_duplicate_sfi(&mut asm);
    assert_eq!(bb.roots().len(), 3);
    let mut bb = BasicBlock::new(vec![sfi_a, sfi_a, sfi_b], 0, None);
    assert_eq!(bb.roots().len(), 3);
    bb.remove_duplicate_sfi(&mut asm);
    assert_eq!(bb.roots().len(), 2);
}
