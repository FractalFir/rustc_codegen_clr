#![allow(clippy::similar_names)]

use crate::{assembly::Assembly, method::Method};
/*
mod locals;
mod op2_combos;
mod op3_combos;
use crate::{
    assembly::Assembly,
    cil::{CILOp, CallSite},
    method::Method,
    r#type::Type,
};

use self::locals::{remove_unused_locals, try_split_locals};
const MAX_PASS: u32 = 10;
pub fn try_inline(caller: &mut Method, inlined: &Method, target: usize) -> bool {
    // Inlining is still sometimes quite buggy.
    if true {
        //return false;
    }

    // Can't yet inline non-empty methods!
    if !inlined.locals().is_empty() {
        return false;
    }

    // Can't yet inline methods with multpile returns
    if inlined
        .get_ops()
        .iter()
        .filter(|op| **op == CILOp::Ret)
        .count()
        > 1
    {
        return false;
    }

    // Can't yet handle inline methods with non-trivial control flow.
    if inlined
        .get_ops()
        .iter()
        .any(|op| matches!(op, CILOp::Label(_)))
    {
        return false;
    }
    // inlined first validation
    if cfg!(debug_assertions) {
        crate::utilis::check_debugable(
            inlined.get_ops(),
            inlined,
            *inlined.sig().output() == Type::Void,
        );
    }
    let arg_beg = caller.locals().len();
    caller.add_local(inlined.sig().output().clone());
    caller.extend_locals(inlined.sig().inputs().iter());
    let mut inlined_call = Vec::new();
    for (index, atype) in inlined.sig().inputs().iter().enumerate() {
        if *atype == Type::Void {
            continue;
        }
        inlined_call.push(CILOp::STLoc((arg_beg + 1 + index) as u32));
    }
    let mut inlined_method_ops: Vec<_> = inlined.get_ops().into();
    inlined_method_ops.iter_mut().for_each(|op| match op {
        CILOp::LDArg(id) => *op = CILOp::LDLoc((arg_beg + 1 + *id as usize) as u32),
        CILOp::LDArgA(id) => *op = CILOp::LDLocA((arg_beg + 1 + *id as usize) as u32),
        CILOp::STArg(id) => *op = CILOp::STArg((arg_beg + 1 + *id as usize) as u32),
        CILOp::LDLoc(_) | CILOp::LDLocA(_) | CILOp::STLoc(_) => {
            todo!("Inlining with locals not supported yet!")
        }
        CILOp::Ret => *op = CILOp::Nop,
        _ => (),
    });
    inlined_call.extend(inlined_method_ops);
    let ops = caller.get_ops();
    // Remove the call
    let preamble = &ops[..target];
    let epilouge = &ops[(target + 1)..];
    let mut new_ops = Vec::with_capacity(ops.len() + inlined_call.len());
    new_ops.extend(preamble.iter().cloned());
    new_ops.extend(inlined_call);
    new_ops.extend(epilouge.iter().cloned());
    // Validate method AFTER inline
    crate::utilis::check_debugable(&new_ops, &new_ops, *caller.sig().output() == Type::Void);
    caller.set_ops(new_ops);
    // Inlining succcedded.
    true
}
fn get_inlline_candidates(method: &Method) -> Vec<(usize, Box<CallSite>)> {
    method
        .get_ops()
        .iter()
        .enumerate()
        .filter_map(|(idx, op)| match op {
            CILOp::Call(callsite) => Some((idx, callsite.clone())),
            _ => None,
        })
        .filter(|(_, site)| site.is_static() && site.class().is_none())
        .collect()
}
fn try_inline_all(method: &mut Method, asm: &Assembly) {
    //Inlining
    let inline_candidates = get_inlline_candidates(method);
    for (target, candidate) in inline_candidates {
        debug_assert_eq!(method.get_ops()[target], CILOp::Call(candidate.clone()));
        let Some(linlined) = asm.methods().find(|method| {
            method.name() == candidate.name()
                && method.sig() == candidate.signature()
                && method.is_static()
        }) else {
            continue;
        };

        // If inline succeds, then the positions of all inline targets will become wrong, and rebuilding of the inline target list becomes necessary.
        if try_inline(method, linlined, target) {
            //try_inline_all(method, asm);
            return;
        }
    }
}*/
//pub fn try_turn_locals_into_bools(method:&Method){}
pub fn opt_method(_: &mut Method, _: &Assembly) {
    /*
    if !*crate::config::OPTIMIZE_CIL {
        return;
    };
    //panic!("opt");
    method.ops_mut().retain(|op| match op {
        CILOp::Call(site) => !site.is_nop(),
        _ => true,
    });
    repalce_const_sizes(method.ops_mut());
    let mut cache = LocalAccessRangeCache::new();
    for pass in 0..MAX_PASS {
        let (ops, locals) = method.ops_n_locals_mut();
        op2_combos::optimize_combos(ops, locals);
        op3_combos::optimize_combos(method.ops_mut());
        op4_combos(method.ops_mut());
        let local_count = method.locals().len();

        match pass % 4 {
            0 => remove_dead_lables(method.ops_mut()),
            2 => try_alias_locals(method.ops_mut(), &mut cache),
            1 | 3 => remove_zombie_sets(method.ops_mut(), local_count),
            _ => panic!(),
        }
        method.ops_mut().retain(|op| *op != CILOp::Nop);
        cache.reset();

        if *crate::config::SPLIT_LOCAL_STRUCTS {
            try_split_locals(method, asm);
        }
        if *crate::config::REMOVE_UNSUED_LOCALS {
            remove_unused_locals(method);
        }
        if *crate::config::INLINE_SIMPLE_FUNCTIONS {
            try_inline_all(method, asm);
        }
        //try_inline_all(method, asm);
    }*/
} /*
  fn repalce_const_sizes(ops: &mut [CILOp]) {
      ops.iter_mut().for_each(|op| {
          if let CILOp::SizeOf(tpe) = op {
              match tpe.as_ref() {
                  Type::U8 | Type::I8 => *op = CILOp::LdcI32(1),
                  _ => (),
              }
          }
      });
  }
  fn remove_zombie_sets(ops: &mut Vec<CILOp>, local_count: usize) {
      //(0..local_count).into_iter().map(|idx|is_local_dead(ops,idx as u32)).collect();//
      let dead_locals: Vec<_> = dead_locals(ops, local_count);
      for idx in 0..ops.len() {
          match ops[idx] {
              CILOp::STLoc(loc) => {
                  if dead_locals[loc as usize] {
                      ops[idx] = CILOp::Pop;
                  }
              }
              _ => (),
          }
      }
  }
  fn remove_dead_lables(ops: &mut Vec<CILOp>) {
      for idx in 0..ops.len() {
          match ops[idx] {
              CILOp::Label(label) => {
                  if is_label_unsused(ops, label) {
                      ops[idx] = CILOp::Nop;
                  }
              }
              _ => (),
          }
      }
  }

  fn op4_combos(ops: &mut [CILOp]) {
      if ops.len() < 4 {
          return;
      }
      // Beq(A) Br(B) Label(A)
      for idx in 0..(ops.len() - 3) {
          let (op1, op2, op3, op4) = (&ops[idx], &ops[idx + 1], &ops[idx + 2], &ops[idx + 3]);
          match (op1, op2, op3, op4) {
              (
                  CILOp::STLoc(b1),
                  CILOp::LDLoc(_) | CILOp::LDArg(_),
                  CILOp::LDLoc(b2),
                  CILOp::Add | CILOp::Mul,
              ) => {
                  if b1 == b2 {
                      let op2 = op2.clone();
                      ops[idx + 1] = op4.clone();
                      ops[idx] = op2;
                      ops[idx + 2] = CILOp::Nop;
                      ops[idx + 3] = CILOp::Nop;
                  }
              }
              (
                  CILOp::STLoc(b1),
                  CILOp::LDLoc(_),
                  CILOp::LDLoc(b2),
                  CILOp::BGe(_) | CILOp::BEq(_) | CILOp::BNe(_) | CILOp::Eq,
              ) => {
                  if b1 == b2 {
                      // b
                      // dup  | b b
                      // stloc(b) | b
                      // ldloc(a) | b a instead of ab
                      // flipped contional
                      let op2 = op2.clone();
                      let op4 = op4.flip_cond();
                      let b = *b1;
                      ops[idx] = CILOp::Dup;
                      ops[idx + 1] = CILOp::STLoc(b);
                      ops[idx + 2] = op2;
                      ops[idx + 3] = op4;
                  }
              }
              (
                  CILOp::LDLoc(_a),
                  CILOp::STLoc(b1),
                  CILOp::LDLoc(c) | CILOp::LDArg(c),
                  CILOp::LDLoc(b2),
              ) => {
                  if b1 == b2 && c != b1 {
                      let op1 = op1.clone();
                      let op3 = op3.clone();
                      let b = *b1;
                      ops[idx] = op3;
                      ops[idx + 1] = op1;
                      ops[idx + 2] = CILOp::Dup;
                      ops[idx + 3] = CILOp::STLoc(b);
                  }
              }
              _ => (),
          }
      }
  }
  /// A "Dead" local is one that is only written into - never read.
  fn is_local_dead(ops: &[CILOp], local: u32) -> bool {
      !ops.iter().any(|op| match op {
          CILOp::LDLoc(loc) => *loc == local,
          CILOp::LDLocA(loc) => *loc == local,
          _ => false,
      })
  }
  fn dead_locals(ops: &[CILOp], locals: usize) -> Vec<bool> {
      let mut locals = vec![true; locals];
      ops.iter().for_each(|op| match op {
          CILOp::LDLoc(loc) | CILOp::LDLocA(loc) => {
              locals[*loc as usize] = false;
          }
          _ => (),
      });
      locals
  }

  /// A "Unused" label is one that is never jumped to
  fn is_label_unsused(ops: &[CILOp], label: u32) -> bool {
      !ops.iter().any(|op| match op {
          CILOp::BEq(target)
          | CILOp::GoTo(target)
          | CILOp::BNe(target)
          | CILOp::BLt(target)
          | CILOp::BGe(target)
          | CILOp::BLe(target)
          | CILOp::BZero(target)
          | CILOp::BTrue(target) => label == *target,
          _ => false,
      })
  }
  #[test]
  fn cond_reordering() {
      let mut ops = [
          CILOp::STLoc(0),
          CILOp::LDLoc(1),
          CILOp::LDLoc(0),
          CILOp::BGe(0),
      ];
      op4_combos(&mut ops);
      assert_eq!(
          ops,
          [CILOp::Dup, CILOp::STLoc(0), CILOp::LDLoc(1), CILOp::BLe(0)]
      );
      //panic!("ops:{ops:?}")
  }
  fn alias_local(src: u32, dst: u32, ops: &mut [CILOp]) {
      ops.iter_mut().for_each(|op| match op {
          CILOp::LDLoc(loc) | CILOp::STLoc(loc) | CILOp::LDLocA(loc) => {
              if *loc == src {
                  *loc = dst;
              }
          }
          _ => (),
      });
  }
  struct LocalAccessRangeCache {
      cache: Vec<Option<Option<Range<usize>>>>,
  }
  impl LocalAccessRangeCache {
      fn new() -> Self {
          Self { cache: Vec::new() }
      }
      fn reset(&mut self) {
          self.cache.iter_mut().for_each(|cached| *cached = None);
      }
      fn extend_to(&mut self, to: u32) {
          while self.cache.len() <= to as usize {
              self.cache.push(None);
          }
      }
      fn get(&mut self, index: u32, ops: &[CILOp]) -> Option<Range<usize>> {
          if self.cache.len() <= index as usize {
              self.extend_to(index);
          }
          if let Some(cached) = &self.cache[index as usize] {
              return cached.clone();
          }
          self.cache[index as usize] = Some(get_local_access_range(index, ops));
          return self.cache[index as usize].as_ref().unwrap().clone();
      }
  }
  fn try_alias_locals(ops: &mut [CILOp], cache: &mut LocalAccessRangeCache) {
      if ops.len() < 2 {
          return;
      }
      for index in 0..(ops.len() - 2) {
          let op1 = &ops[index];
          let op2 = &ops[index + 1];
          let op3 = &ops[index + 2];
          if let (CILOp::LDLoc(loc1), CILOp::STLoc(loc2)) = (op1, op2) {
              if loc1 == loc2 {
                  continue;
              }
              if could_local_ptr_escape(*loc1, ops) {
                  continue;
              }
              let Some(loc1_range) = cache.get(*loc1, ops) else {
                  continue;
              };
              let Some(loc2_range) = cache.get(*loc2, ops) else {
                  continue;
              };

              // Ranges don't overlap, use simple aliasing
              if !do_ranges_overlap(&loc1_range, &loc2_range) {
                  //println!("{loc2} will now be {loc1}!");
                  alias_local(*loc2, *loc1, ops);
                  continue;
              }
          }
          if let (CILOp::LDLoc(loc1), CILOp::Dup, CILOp::STLoc(loc2)) = (op1, op2, op3) {
              if loc1 == loc2 {
                  continue;
              }
              if could_local_ptr_escape(*loc1, ops) {
                  continue;
              }
              let Some(loc1_range) = cache.get(*loc1, ops) else {
                  continue;
              };
              let Some(loc2_range) = cache.get(*loc2, ops) else {
                  continue;
              };

              // Ranges don't overlap, use simple aliasing
              if !do_ranges_overlap(&loc1_range, &loc2_range) {
                  //println!("{loc2} will now be {loc1}!");
                  alias_local(*loc2, *loc1, ops);
                  continue;
              }
          }
      }
  }
  fn do_ranges_overlap(range1: &Range<usize>, range2: &Range<usize>) -> bool {
      range1.start <= range2.end && range2.start <= range1.end
  }
  fn get_local_access_range(local: u32, ops: &[CILOp]) -> Option<Range<usize>> {
      let mut start = ops.len();
      let mut end = 0;
      for (index, op) in ops.iter().enumerate() {
          match op {
              CILOp::LDLoc(loc) | CILOp::STLoc(loc) | CILOp::LDLocA(loc) => {
                  if *loc == local {
                      start = start.min(index);
                      end = end.max(index);
                  }
              }
              _ => (),
          }
      }
      if start > end {
          eprintln!("Can't get range!");
          None
      } else {
          Some(start..end)
      }
  }
  /// Checks if it is possible for a pointer to a local to escape.
  fn could_local_ptr_escape(local: u32, ops: &[CILOp]) -> bool {
      for (index, op) in ops.iter().enumerate() {
          if let CILOp::LDLocA(loc) = op {
              if *loc == local {
                  assert!(
                      index + 2 < ops.len(),
                      "ERROR: malformed method. LDLocA must be followed by at least 2 ops."
                  );
                  let op2 = &ops[index + 1];
                  let op3 = &ops[index + 2];
                  if let CILOp::LDField(_) = op2 {
                      continue;
                  }
                  if let CILOp::STField(_) = op3 {
                      continue;
                  }
                  return true;
              }
          }
      }
      false
  }
   */
