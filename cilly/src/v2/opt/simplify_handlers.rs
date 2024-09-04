#![allow(dead_code, unused_variables)]
use fxhash::{FxHashMap, FxHashSet};

use crate::v2::{Assembly, BasicBlock, CILRoot};

use super::OptFuel;
fn block_with_id(blocks: &[BasicBlock], id: u32) -> Option<&BasicBlock> {
    blocks.iter().find(|block| block.block_id() == id)
}
fn block_targets<'a, 'asm: 'a>(
    block: &'a BasicBlock,
    asm: &'asm Assembly,
) -> impl Iterator<Item = u32> + 'a {
    block.roots().iter().filter_map(|root| {
        if let CILRoot::Branch(info) = asm.get_root(*root) {
            let (target, sub_target, _) = info.as_ref();
            //Some(*sub_target)
            //(eprintln!("{target} {sub_target}");
            if *sub_target == 0 {
                Some(*target)
            } else {
                Some(*sub_target)
            }
        } else {
            None
        }
    })
}
fn block_gc(blocks: &mut Vec<BasicBlock>, asm: &Assembly) {
    //debug_assert!(crate::utilis::is_sorted(bbs.iter(),|a,b|a.id + 1 == b.id));
    let mut alive: FxHashSet<u32> = FxHashSet::default();
    let mut resurecting = FxHashSet::default();
    let mut to_resurect = FxHashSet::default();
    to_resurect.insert(blocks[0].block_id());
    while !to_resurect.is_empty() {
        alive.extend(&resurecting);
        resurecting.clear();
        resurecting.extend(&to_resurect);
        to_resurect.clear();
        for target in resurecting
            .iter()
            .filter_map(|bb| block_with_id(blocks, *bb))
            .flat_map(|bb| block_targets(bb, asm).collect::<Vec<_>>())
        {
            //eprintln!("Block {target} is alive.");
            if !alive.contains(&target) && !resurecting.contains(&target) {
                to_resurect.insert(target);
            }
        }
    }
    alive.extend(&resurecting);
    *blocks = blocks
        .iter()
        .filter(|bb| alive.contains(&bb.block_id()))
        .cloned()
        .collect();
}
pub fn simplify_bbs(handler: Option<&mut Vec<BasicBlock>>, asm: &mut Assembly, fuel: &mut OptFuel) {
    let Some(blocks) = handler else { return };
    let direct_jumps: FxHashMap<_, Option<(u32, u32)>> = blocks
        .iter()
        .map(|block| (block.block_id(), block.is_direct_jump(asm)))
        .collect();
    let rethrows: FxHashMap<_, bool> = blocks
        .iter()
        .map(|block| (block.block_id(), block.is_only_rethrow(asm)))
        .collect();
    for root in blocks
        .iter_mut()
        .flat_map(|block| block.roots_mut().iter_mut())
    {
        let CILRoot::Branch(info) = asm.get_root(*root) else {
            continue;
        };
        /*let (target, sub_target, cond) = info.as_ref();
        // Sub target of 0, look up by the target
        let jump = if *sub_target == 0 {
            direct_jumps.get(target)
        } else {
            direct_jumps.get(sub_target)
        };
        let Some(jump) = jump else {
            continue;
        };
        let Some((target, sub_target)) = jump else {
            let rethrow = if *sub_target == 0 {
                rethrows.get(target).unwrap()
            } else {
                rethrows.get(sub_target).unwrap()
            };
            if *rethrow && fuel.consume(1) {
                *root = asm.alloc_root(CILRoot::ReThrow);
            }
            continue;
        };
        if fuel.consume(1) {
            *root = asm.alloc_root(CILRoot::Branch(Box::new((
                *target,
                *sub_target,
                cond.clone(),
            ))));
        }*/
    }

    //block_gc(handler, asm);
}
