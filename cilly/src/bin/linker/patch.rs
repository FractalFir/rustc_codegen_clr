use cilly::{
    cilnode::PtrCastRes,
    IString,
    {asm::MissingMethodPatcher, Assembly, CILNode, CILRoot, Int, MethodRef, Type},
};

pub fn call_alias(
    overrides: &mut MissingMethodPatcher,
    asm: &mut Assembly,
    name: impl Into<IString>,
    call: MethodRef,
) {
    overrides.insert(
        asm.alloc_string(name),
        Box::new(move |original, asm| {
            let method_ref = asm.alloc_methodref(call.clone());
            let inputs: Box<[_]> = asm[call.sig()].inputs().into();
            let original_inputs: Box<[_]> =
                asm[asm[original].sig()].inputs().into();
            let args:Box<_> = inputs
                .iter()
                .zip(original_inputs.iter())
                .enumerate()
                .map(|(arg, (target_type, original_tpe))| {
                    if target_type == original_tpe {
                        asm.alloc_node(CILNode::LdArg(arg as u32))
                    } else {
                        match (target_type, original_tpe) {
                            (
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
                                Type::ClassRef(_),
                            ) => {
                                // Transmute to a pointer
                                let ptr_address = asm.alloc_node(CILNode::LdArgA(arg as u32));
                                let tpe = asm.alloc_type(*target_type);
                                let ptr_address =  asm.alloc_node(CILNode::PtrCast(ptr_address, Box::new(PtrCastRes::Ptr(tpe))));
                                asm.alloc_node(CILNode::LdInd {
                                    addr: ptr_address,
                                    tpe,
                                    volatile: false,
                                })
                            }
                            (
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize),
                                Type::Int(Int::U64),
                            ) => {
                                let arg = asm.alloc_node(CILNode::LdArg(arg as u32));

                                asm.alloc_node(CILNode::IntCast {
                                    input: arg,
                                    target: Int::USize,
                                    extend: cilly::cilnode::ExtendKind::ZeroExtend,
                                })
                            }
                            (
                                Type::Int(int @ (Int::ISize | Int::USize)),
                                Type::Int(Int::ISize | Int::USize)
                            )  => {
                                let src =  asm.alloc_node(CILNode::LdArg(arg as u32));
                                asm.alloc_node(CILNode::IntCast { input: src, target:*int, extend:cilly::cilnode::ExtendKind::ZeroExtend })
                            },
                            (
                                Type::Ptr(dst),
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_) ,
                            )=>{
                                let arg = asm.alloc_node(CILNode::LdArg(arg as u32));
                                asm.alloc_node(CILNode::PtrCast(arg, Box::new(PtrCastRes::Ptr(*dst))))
                            }
                            (
                                Type::FnPtr(dst) ,
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_) ,
                            )=>{
                                let arg = asm.alloc_node(CILNode::LdArg(arg as u32));
                                asm.alloc_node(CILNode::PtrCast(arg, Box::new(PtrCastRes::FnPtr(*dst))))
                            }
                            (
                                Type::Int(Int::ISize | Int::USize),
                                Type::Ptr(_) | Type::FnPtr(_),
                            )  => {
                                asm.alloc_node(CILNode::LdArg(arg as u32))
                            },
                            (Type::Int(Int::I64),Type::Int(Int::U64)) => asm.alloc_node(CILNode::LdArg(arg as u32)),
                            _ => todo!("can't auto convert {original_tpe:?} to {target_type:?} when autogenrating wrappers."),
                        }
                    }
                })
                .collect();
            if *asm[call.sig()].output() == Type::Void {
                let call = asm.alloc_root(CILRoot::call(method_ref, args));
                let ret = asm.alloc_root(CILRoot::VoidRet);
                cilly::MethodImpl::MethodBody {
                    blocks: vec![cilly::BasicBlock::new(vec![call, ret], 0, None)],
                    locals: vec![],
                }
            } else {
                let ret_value = asm.alloc_node(CILNode::call(method_ref, args));
                let ret = asm.alloc_root(CILRoot::Ret(ret_value));
                cilly::MethodImpl::MethodBody {
                    blocks: vec![cilly::BasicBlock::new(vec![ret], 0, None)],
                    locals: vec![],
                }
            }
        }),
    );
}
