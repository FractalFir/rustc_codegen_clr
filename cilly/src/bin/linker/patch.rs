use cilly::{
    v2::{asm::MissingMethodPatcher, Assembly, CILNode, CILRoot, Int, MethodRef, Type},
    IString,
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
            let args = inputs
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
                                asm.alloc_node(CILNode::LdInd {
                                    addr: ptr_address,
                                    tpe,
                                    volitale: false,
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
                                    extend: cilly::v2::cilnode::ExtendKind::ZeroExtend,
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
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
                                Type::Ptr(_) | Type::Int(Int::ISize | Int::USize) | Type::FnPtr(_),
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
                let call = asm.alloc_root(CILRoot::Call(Box::new((method_ref, args))));
                let ret = asm.alloc_root(CILRoot::VoidRet);
                cilly::v2::MethodImpl::MethodBody {
                    blocks: vec![cilly::v2::BasicBlock::new(vec![call, ret], 0, None)],
                    locals: vec![],
                }
            } else {
                let ret_value = asm.alloc_node(CILNode::Call(Box::new((method_ref, args))));
                let ret = asm.alloc_root(CILRoot::Ret(ret_value));
                cilly::v2::MethodImpl::MethodBody {
                    blocks: vec![cilly::v2::BasicBlock::new(vec![ret], 0, None)],
                    locals: vec![],
                }
            }
        }),
    );
}
