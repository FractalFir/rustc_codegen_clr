use std::num::NonZeroU8;

use crate::{
    access_modifier::AccessModifer,
    call, conv_isize, conv_usize,
    v2::{cilnode::MethodKind, Assembly, FnSig, Int, MethodRef},
    Access, BasicBlock, CILNode, CILRoot, Const, MethodDef, MethodDefIdx, MethodImpl, Type,
};

/// Creates a wrapper method around entypoint represented by `MethodRefIdx`
pub fn wrapper(entrypoint: MethodRef, asm: &mut Assembly) -> MethodDefIdx {
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_idx = asm.alloc_type(uint8_ptr);
    let uint8_ptr_ptr = asm.nptr(uint8_ptr);

    let entry_sig = asm[entrypoint.sig()].clone();
    let main_module = asm.main_module();
    let entrypoint_name = asm.alloc_string("entrypoint");
    let entrypoint = asm.alloc_methodref(entrypoint);
    if entry_sig.inputs() == [Type::Int(Int::ISize), uint8_ptr_ptr]
        && entry_sig.output() == &Type::Int(Int::ISize)
    {
        let string = asm.alloc_type(Type::PlatformString);
        let sig = asm.sig(
            ([Type::PlatformArray {
                elem: string,
                dims: NonZeroU8::new(1).unwrap(),
            }]),
            Type::Void,
        );
        let tcctor = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string(".tcctor"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let tcctor = asm.alloc_methodref(tcctor);
        let static_init = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string("static_init"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let static_init = asm.alloc_methodref(static_init);
        let argv = asm.alloc_node(Const::ISize(0_i64));
        let argv = asm.alloc_node(CILNode::PtrCast(
            argv,
            Box::new(crate::cilnode::PtrCastRes::Ptr(uint8_ptr_idx)),
        ));
        let args = [asm.alloc_node(Const::ISize(0_i64)), argv];

        let call_main = CILNode::Call(Box::new(((entrypoint), args.into())));
        let call_main = asm.alloc_node(call_main);
        let blocks = vec![BasicBlock::new(
            vec![
                asm.alloc_root(CILRoot::Call(Box::new((tcctor, [].into())))),
                asm.alloc_root(CILRoot::Call(Box::new((static_init, [].into())))),
                asm.alloc_root(CILRoot::Pop(call_main)),
                asm.alloc_root(CILRoot::VoidRet),
            ],
            2,
            None,
        )];
        let mimpl = MethodImpl::MethodBody {
            blocks,
            locals: vec![],
        };
        let method = MethodDef::new(
            Access::Extern,
            main_module,
            entrypoint_name,
            sig,
            MethodKind::Static,
            mimpl,
            vec![Some(asm.alloc_string("args"))],
        );

        asm.new_method(method)
    } else if entry_sig.inputs().is_empty() && entry_sig.output() == &Type::Void {
        let sig = asm.sig([], Type::Void);
        let tcctor = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string(".tcctor"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let tcctor = asm.alloc_methodref(tcctor);
        let static_init = MethodRef::new(
            *asm.main_module(),
            asm.alloc_string("static_init"),
            asm.sig([], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        let static_init = asm.alloc_methodref(static_init);
        let blocks = vec![BasicBlock::new(
            vec![
                asm.alloc_root(CILRoot::Call(Box::new((tcctor, [].into())))),
                asm.alloc_root(CILRoot::Call(Box::new(((static_init), [].into())))),
                asm.alloc_root(CILRoot::Call(Box::new((entrypoint, [].into())))),
                //CILRoot::debug(&format!("Preparing to execute the main program.")).into(),
                asm.alloc_root(CILRoot::VoidRet),
            ],
            0,
            None,
        )];
        let mut method = MethodDef::new(
            Access::Extern,
            main_module,
            entrypoint_name,
            sig,
            MethodKind::Static,
            crate::MethodImpl::MethodBody {
                blocks,
                locals: vec![],
            },
            vec![],
        );

        asm.new_method(method)
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
