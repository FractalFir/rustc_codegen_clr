use crate::assembly::MethodCompileCtx;
use cilly::{
    call, call_virt, cil_node::CILNode, cil_root::CILRoot, cilnode::MethodKind, conv_u32,
    v2::ClassRef, Int, MethodRef, Type,
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_middle::{mir::Place, ty::Instance};
pub fn type_id<'tcx>(
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    let type_type = ClassRef::type_type(ctx);
    let runtime_handle = ClassRef::runtime_type_hadle(ctx);
    let sig = ctx.sig([runtime_handle.into()], type_type);
    let gethash_sig = ctx.sig([type_type.into()], Type::Int(Int::I32));
    let op_implict = MethodRef::new(
        ClassRef::uint_128(ctx),
        ctx.alloc_string("op_Implicit"),
        ctx.sig([Type::Int(Int::U32)], Type::Int(Int::U128)),
        MethodKind::Static,
        vec![].into(),
    );
    let get_hash_code = MethodRef::new(
        ClassRef::object(ctx),
        ctx.alloc_string("GetHashCode"),
        gethash_sig,
        MethodKind::Virtual,
        vec![].into(),
    );
    let get_type_handle = MethodRef::new(
        type_type,
        ctx.alloc_string("GetTypeFromHandle"),
        sig,
        MethodKind::Static,
        vec![].into(),
    );
    place_set(
        destination,
        call!(
            ctx.alloc_methodref(op_implict),
            [conv_u32!(call_virt!(
                ctx.alloc_methodref(get_hash_code),
                [call!(
                    ctx.alloc_methodref(get_type_handle),
                    [CILNode::LDTypeToken(tpe.into())]
                )]
            ))]
        ),
        ctx,
    )
}
