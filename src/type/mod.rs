/// A representation of a primitve type or a reference.
pub mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
pub mod type_def;

pub mod tycache;

pub use r#type::*;
pub use tycache::*;
pub use type_def::*;
use rustc_middle::ty::{Ty,TyKind,GenericArg,UintTy,IntTy};
use crate::IString;
pub fn mangle_ty(ty:Ty)->std::borrow::Cow<'static,str>{
    match ty.kind(){
        TyKind::Bool=>"B".into(),
        TyKind::Slice(inner)=>format!("Sl{}",mangle_ty(*inner)).into(),
        TyKind::Str=>"Str".into(),
        TyKind::Uint(uint)=> match uint{
            UintTy::U8=>"U8",
            UintTy::U16=>"U16",
            UintTy::U32=>"U32",
            UintTy::U64=>"U64",
            UintTy::U128=>"U128",
            UintTy::Usize=>"USize",
        }.into(),
        TyKind::Int(int)=> match int{
            IntTy::I8=>"I8",
            IntTy::I16=>"I16",
            IntTy::I32=>"I32",
            IntTy::I64=>"I64",
            IntTy::I128=>"I128",
            IntTy::Isize=>"ISize",
        }.into(),
        TyKind::Ref(_region, inner, _mut) =>format!("Ref{}",mangle_ty(*inner)).into(),
        TyKind::Adt(def,subst)=>{
            let name = crate::utilis::adt_name(&def);
            if is_name_magic(name.as_ref()) {
                todo!("Can't yet handle interop-related generic types.");
            }
            mangle_susbt(&name,subst)
        }
        _=>todo!("Can't yet mangle type {ty:?}"),
    }
}
fn mangle_elem(elem:&GenericArg)->std::borrow::Cow<'static,str>{
    if let Some(tpe)= elem.as_type(){
        return mangle_ty(tpe);
    }
    if let Some(tpe)= elem.as_const(){
        todo!("Can't mangle consts!");
    }
    return "".into();
}
pub(self) fn mangle_susbt(name:&str,subst:&[GenericArg])->std::borrow::Cow<'static,str>{
    let mut result:String = name.into();
    for generic in subst{
        result.push_str(&mangle_elem(&generic));
    }
    result.into()
}
