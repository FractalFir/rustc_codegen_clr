use serde::{Deserialize, Serialize};
use rustc_middle::ty::{Ty, TyCtxt,TyKind};
use crate::{r#type::{Type, DotnetTypeRef}, IString, access_modifier::AccessModifer};
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TypeDef {
    access:AccessModifer,
    name:IString,
    inner_types: Vec<Self>,
    fields: Vec<(IString,Type)>,
    gargc:u32,
    extends:Option<DotnetTypeRef>,
}
impl TypeDef{
    pub fn gargc(&self)->u32{
        self.gargc
    }
    pub fn name(&self)->&str{
        &self.name
    }
    pub fn extends(&self)->Option<&DotnetTypeRef>{
        self.extends.as_ref()
    }
    pub fn nameonly(name:&str)->Self{
        Self{access:AccessModifer::Public,name:name.into(),inner_types:vec![],fields:vec![],gargc:0,extends:None}
    }
    pub fn from_ty<'tycxt>(ty:Ty<'tycxt>,ctx:TyCtxt<'tycxt>)->Vec<Self>{
        println!("defs {ty:?}");
        match ty.kind(){
            TyKind::Adt(adt_def,subst)=>{
                let name = crate::utilis::adt_name(adt_def);
                let gargc = subst.len() as u32;
                let access = AccessModifer::Public;
                vec![Self{access,name,inner_types:vec![],fields:vec![],gargc,extends:None}]
            },
            TyKind::Ref(_region, inner, _mut)=>Self::from_ty(*inner,ctx),
            TyKind::RawPtr(inner_and_mut)=>Self::from_ty(inner_and_mut.ty,ctx),
            _=>vec![],
        }
    }
}
