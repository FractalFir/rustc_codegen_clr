#![allow(dead_code, unused_variables)]
use fxhash::{FxHashMap, FxHashSet};

use crate::v2::{Assembly, BasicBlock, CILRoot};

use super::{OptFuel, SideEffectInfoCache};
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
fn blockid_from_jump(target: u32, sub_target: u32) -> u32 {
    if sub_target == 0 {
        target
    } else {
        sub_target
    }
}
pub fn simplify_bbs(
    handler: Option<&mut Vec<BasicBlock>>,
    asm: &mut Assembly,
    fuel: &mut OptFuel,
    cache: &mut SideEffectInfoCache,
) {
    let Some(blocks) = handler else { return };
    let direct_jumps: FxHashMap<_, Option<(u32, u32)>> = blocks
        .iter()
        .map(|block| (block.block_id(), block.is_direct_jump(asm)))
        .collect();
    let rethrows: FxHashMap<_, bool> = blocks
        .iter()
        .map(|block| (block.block_id(), block.is_only_rethrow(asm)))
        .collect();
    let mut root_iter = blocks
        .iter_mut()
        .flat_map(|block| block.roots_mut().iter_mut())
        .peekable();
    while let Some(root) = root_iter.next() {
        let CILRoot::Branch(info) = asm.get_root(*root) else {
            continue;
        };
        let (target, sub_target, cond) = info.as_ref();
        // Sub target of 0, look up by the target
        let jump = direct_jumps.get(&blockid_from_jump(*target, *sub_target));
        let Some(jump) = jump else {
            continue;
        };
        let Some((target, sub_target)) = jump else {
            // Check that this jump is unconditonal, or the next root is a rethrow!
            if let Some(cond) = cond {
                if root_iter.peek().map(|root| asm.get_root(**root)) != Some(&CILRoot::ReThrow) {
                    continue;
                }
                // If some args have side effects, this optimization has to replace this branch with pops. TODO: implement that.
                if cond
                    .nodes()
                    .into_iter()
                    .any(|node| cache.has_side_effects(node, asm))
                {
                    continue;
                }
            }
            // TODO: Correctnesss:Check if this root's tree has no side effects!
            let rethrow = rethrows
                .get(&blockid_from_jump(*target, *sub_target))
                .unwrap();
            if *rethrow && fuel.consume(1) {
                *root = asm.alloc_root(CILRoot::ReThrow);
            }
            continue;
        };
        /*
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
#[test]
fn find_block() {
    let blocks = vec![];
    assert!(block_with_id(&blocks, 0).is_none());
    let blocks = vec![
        BasicBlock::new(vec![], 0, None),
        BasicBlock::new(vec![], 1, None),
    ];
    assert!(block_with_id(&blocks, 0).is_some());
    assert!(block_with_id(&blocks, 1).is_some());
    assert!(block_with_id(&blocks, 2).is_none());
}
#[test]
fn targets() {
    let mut asm = Assembly::default();
    let block = BasicBlock::new(vec![], 0, None);
    assert_eq!(block_targets(&block, &asm).count(), 0);
    let nop = asm.alloc_root(CILRoot::Nop);
    let block = BasicBlock::new(vec![nop], 0, None);
    assert_eq!(block_targets(&block, &asm).count(), 0);
    let goto = asm.alloc_root(CILRoot::Branch(Box::new((0, 0, None))));
    let block = BasicBlock::new(vec![nop, goto, nop], 0, None);
    assert_eq!(block_targets(&block, &asm).count(), 1);
}
#[test]
fn blockid() {
    assert_eq!(blockid_from_jump(0, 0), 0);
    assert_eq!(blockid_from_jump(2, 1), 1);
    assert_eq!(blockid_from_jump(1, 2), 2);
    assert_eq!(blockid_from_jump(2, 0), 2);
}
