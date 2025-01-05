use crate::v2::{
    asm::MissingMethodPatcher, cilnode::MethodKind, cilroot::BranchCond, BasicBlock, BinOp,
    CILNode, CILRoot, ClassRef, Const, Int, MethodImpl, MethodRef, Type,
};

use super::{
    super::{Assembly, NodeIdx},
    math::{int_max, int_min},
};
/// Emulates operations on bytes using operations on int32s. Enidianess dependent, can cause segfuaults when used on a page boundary.
/// TODO: remove when .NET 9 is out.
pub fn emulate_uint8_cmp_xchng(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    generate_atomic(
        asm,
        patcher,
        "cmpxchng8",
        |asm, prev, arg, _| {
            // 1st, mask the previous value
            let prev_mask = asm.alloc_node(Const::I32(0xFFFF_FF00_u32 as i32));
            let prev = asm.alloc_node(CILNode::BinOp(prev, prev_mask, BinOp::And));
        
            asm.alloc_node(CILNode::BinOp(prev, arg, BinOp::Or))
        },
        Int::I32,
    );
    generate_atomic(
        asm,
        patcher,
        "cmpxchng16",
        |asm, prev, arg, _| {
            // 1st, mask the previous value
            let prev_mask = asm.alloc_node(Const::I32(0xFFFF_0000_u32 as i32));
            let prev = asm.alloc_node(CILNode::BinOp(prev, prev_mask, BinOp::And));
          
            asm.alloc_node(CILNode::BinOp(prev, arg, BinOp::Or))
        },
        Int::I32,
    );
    let name = asm.alloc_string("atomic_xchng_u8");
    let generator = move |_, asm: &mut Assembly| {
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        let ldloc_0 = asm.alloc_node(CILNode::LdLoc(0));
        let uint8_idx = asm.alloc_type(Type::Int(Int::U8));
        // Load value at addr 0 and write it to tmp
        let arg0_val = asm.alloc_node(CILNode::LdInd {
            addr: ldarg_0,
            tpe: uint8_idx,
            volatile: true,
        });
        let set_tmp = asm.alloc_root(CILRoot::StLoc(0, arg0_val));
        // Copy arg1 to addr0
        let copy_arg1 = asm.alloc_root(CILRoot::StInd(Box::new((
            ldarg_0,
            ldarg_1,
            Type::Int(Int::U8),
            true,
        ))));
        let ret = asm.alloc_root(CILRoot::Ret(ldloc_0));
        MethodImpl::MethodBody {
            blocks: vec![BasicBlock::new(vec![set_tmp, copy_arg1, ret], 0, None)],
            locals: vec![(None, uint8_idx)],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn compare_exchange(asm:&mut Assembly,int:Int,addr:NodeIdx, value:NodeIdx, comaprand:NodeIdx)->NodeIdx{
    match int.size().unwrap_or(8){
        // u16 is buggy :(. TODO: fix it.
        1 | 2=>{
            let compare_exchange = asm.alloc_string("atomic_cmpxchng8_i32");

            let i32 = Type::Int(int);
            let i32_ref = asm.nref(i32);
            let cmpxchng_sig = asm.sig([i32_ref, i32, i32], i32);
            let main_mod = asm.main_module();
            let mref = asm.alloc_methodref(MethodRef::new(
                *main_mod,
                compare_exchange,
                cmpxchng_sig,
                MethodKind::Static,
                vec![].into(),
            ));
            let cast_value = asm.alloc_node(CILNode::IntCast { input: value, target: Int::I32, extend: crate::cilnode::ExtendKind::ZeroExtend });
            let cast_comparand = asm.alloc_node(CILNode::IntCast { input: comaprand, target: Int::I32, extend: crate::cilnode::ExtendKind::ZeroExtend });
            let addr = asm.alloc_node(CILNode::RefToPtr(addr));
            let i32_tidx = asm.alloc_type(Type::Int(Int::I32));
            let addr = asm.alloc_node(CILNode::PtrCast(addr, Box::new(crate::cilnode::PtrCastRes::Ptr(i32_tidx))));
            let res = asm.alloc_node(CILNode::Call(Box::new((
                mref,
                Box::new([addr, cast_value, cast_comparand]),
            ))));
            asm.alloc_node(CILNode::IntCast { input: res, target: int, extend: crate::cilnode::ExtendKind::ZeroExtend })
        }
        4..=8 => {
            let compare_exchange = asm.alloc_string("CompareExchange");

            let tpe = Type::Int(int);
            let tref = asm.nref(tpe);
            let cmpxchng_sig = asm.sig([tref, tpe, tpe], tpe);
            let interlocked = ClassRef::interlocked(asm);
            let mref = asm.alloc_methodref(MethodRef::new(
                interlocked,
                compare_exchange,
                cmpxchng_sig,
                MethodKind::Static,
                vec![].into(),
            ));
            
            asm.alloc_node(CILNode::Call(Box::new((
                mref,
                Box::new([addr, value, comaprand]),
            ))))
        }
        _=>todo!("Can't cmpxchng {int:?}")
    }
   
}
pub fn generate_atomic(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    op_name: &str,
    op: impl Fn(&mut Assembly, NodeIdx, NodeIdx, Int) -> NodeIdx + 'static,
    int: Int,
) {
    let name = asm.alloc_string(format!("atomic_{op_name}_{int}", int = int.name()));
    let generator = move |_, asm: &mut Assembly| {
        // Common ops
        let ldloc_0 = asm.alloc_node(CILNode::LdLoc(0));
        let ldloc_1 = asm.alloc_node(CILNode::LdLoc(1));
        let ldarg_0 = asm.alloc_node(CILNode::LdArg(0));
        let ldarg_1 = asm.alloc_node(CILNode::LdArg(1));
        // Types for which this atomic is implemented

        // The OP of this atomic
        let op = op(asm, ldloc_0, ldarg_1, int);
        let call = compare_exchange(asm, int, ldarg_0, op, ldloc_0);

        let tpe = Type::Int(int);
        let zero = asm.alloc_node(int.zero());
        let entry_block = vec![
            asm.alloc_root(CILRoot::StLoc(1, zero)),
            asm.alloc_root(CILRoot::Branch(Box::new((1, 0, None)))),
        ];
        let loop_block = vec![
            asm.alloc_root(CILRoot::StLoc(0, ldloc_1)),
            asm.alloc_root(CILRoot::StLoc(1, call)),
            asm.alloc_root(CILRoot::Branch(Box::new((
                0,
                1,
                Some(BranchCond::Ne(ldloc_0, ldloc_1)),
            )))),
            asm.alloc_root(CILRoot::Branch(Box::new((2, 0, None)))),
        ];
        let exit_block = vec![asm.alloc_root(CILRoot::Ret(ldloc_0))];
        MethodImpl::MethodBody {
            blocks: vec![
                BasicBlock::new(entry_block, 0, None),
                BasicBlock::new(loop_block, 1, None),
                BasicBlock::new(exit_block, 2, None),
            ],
            locals: vec![(None, asm.alloc_type(tpe)), (None, asm.alloc_type(tpe))],
        }
    };
    patcher.insert(name, Box::new(generator));
}
pub fn generate_atomic_for_ints(
    asm: &mut Assembly,
    patcher: &mut MissingMethodPatcher,
    op_name: &str,
    op: impl Fn(&mut Assembly, NodeIdx, NodeIdx, Int) -> NodeIdx + 'static + Clone,
) {
    const ATOMIC_INTS: [Int; 10] = [
        Int::U8,
        Int::I8,
        Int::U16,
        Int::I16,
        Int::U32,
        Int::U64,
        Int::USize,
        Int::I32,
        Int::I64,
        Int::ISize,
    ];
    for int in ATOMIC_INTS {
        generate_atomic(asm, patcher, op_name, op.clone(), int);
    }
}
/// Adds all the builitn atomic functions to the patcher, allowing for their use.
pub fn generate_all_atomics(asm: &mut Assembly, patcher: &mut MissingMethodPatcher) {
    generate_atomic_for_ints(asm, patcher, "add", |asm, lhs, rhs, _| {
        asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::Add))
    });
    generate_atomic_for_ints(asm, patcher, "sub", |asm, lhs, rhs, _| {
        asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::Sub))
    });
    // XOR
    generate_atomic_for_ints(asm, patcher, "xor", |asm, lhs, rhs, _| {
        asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::XOr))
    });
    // NAND
    generate_atomic_for_ints(asm, patcher, "nand", |asm, lhs, rhs, _| {
        let and = asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::And));
        asm.alloc_node(CILNode::UnOp(and, crate::v2::cilnode::UnOp::Not))
    });
    // Max
    generate_atomic_for_ints(asm, patcher, "max", int_max);
    // Max
    generate_atomic_for_ints(asm, patcher, "min", int_min);
    // Emulates 1 byte compare exchange
    emulate_uint8_cmp_xchng(asm, patcher);
    for int in [Int::ISize, Int::USize] {
        generate_atomic(
            asm,
            patcher,
            "or",
            |asm, lhs, rhs, _| asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::Or)),
            int,
        );
        generate_atomic(
            asm,
            patcher,
            "and",
            |asm, lhs, rhs, _| asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::And)),
            int,
        );
        generate_atomic(
            asm,
            patcher,
            "add",
            |asm, lhs, rhs, _| asm.alloc_node(CILNode::BinOp(lhs, rhs, BinOp::Add)),
            int,
        );
    }
}
/*
  .method public hidebysig static
        uint32 atomic_xor (
            uint32& addr,
            uint32 xorand
        ) cil managed
    {
        // Method begins at RVA 0x2050
        // Code size 25 (0x19)
        .maxstack 3
        .locals  (
            [0] uint32 addr_val,
            [1] uint32 got
        )


        // loop start (head: IL_0013)
            IL_0006: ldloc.1
            IL_0007: stloc.0

            IL_0008:  ldarg.0
            IL_0009:   ldloc.0
            IL_000a:   ldarg.1
            IL_000b:  xor
            IL_000c:  ldloc.0
            IL_000d: call uint32 [System.Threading]System.Threading.Interlocked::CompareExchange(uint32&, uint32, uint32)
            IL_0012: stloc.1

            IL_0013: ldloc.0
            IL_0014: ldloc.1
            IL_0015: bne.un.s IL_0006
        // end loop
        IL_0017: ldloc.0
        IL_0018: ret
    } // end of method Tmp::atomic_xor

*/
