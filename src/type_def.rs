use serde::{Deserialize, Serialize};
use rustc_middle::ty::{Ty, TyCtxt,TyKind,AdtDef,GenericArg,AdtKind,List};
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
    pub fn set_generic_count(&mut self,generic_count:u32){
        self.gargc = generic_count;
    }
    pub fn gargc(&self)->u32{
        self.gargc
    }
    pub fn name(&self)->&str{
        &self.name
    }
    pub fn extends(&self)->Option<&DotnetTypeRef>{
        self.extends.as_ref()
    }
    pub fn fields(&self)->&[(IString,Type)]{
        &self.fields
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
                match adt_def.adt_kind(){
                    AdtKind::Struct=>Self::struct_from_adt(ty,adt_def,subst,ctx),
                    AdtKind::Enum=>Self::enum_from_adt(ty,adt_def,subst,ctx),
                    _=>vec![Self{access,name,inner_types:vec![],fields:vec![],gargc,extends:None}],
                }
                
            },
            TyKind::Ref(_region, inner, _mut)=>Self::from_ty(*inner,ctx),
            TyKind::RawPtr(inner_and_mut)=>Self::from_ty(inner_and_mut.ty,ctx),
            TyKind::Slice(inner)=>Self::from_ty(*inner,ctx),
            _=>vec![],
        }
    }
    fn struct_from_adt<'tcx>(original:Ty<'tcx>,adt_def:&AdtDef<'tcx>,subst:&'tcx List<GenericArg<'tcx>>,ctx:TyCtxt<'tcx>)->Vec<Self>{
        let name = crate::utilis::adt_name(adt_def);
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        for field in adt_def.all_fields(){
            //Add generic types of fields
            {
                let resolved_field_ty =  field.ty(ctx,subst);
                //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
                if resolved_field_ty != original{
                    res.extend(Self::from_ty(resolved_field_ty,ctx));
                }
            }
            //rustc_middle::ty::List::empty()
            let ty = ctx.type_of(field.did).instantiate_identity();
            let ty = Type::from_ty(ty,ctx);
            let name = escape_type_name(field.name.to_string().into());
            fields.push((name,ty));
        }
        res.push(Self{access,name,inner_types:vec![],fields,gargc,extends:None});
        res
    }
    fn enum_from_adt<'tcx>(original:Ty<'tcx>,adt_def:&AdtDef<'tcx>,subst:&'tcx List<GenericArg<'tcx>>,ctx:TyCtxt<'tcx>)->Vec<Self>{
        let name = crate::utilis::adt_name(adt_def);
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        //let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        for field in adt_def.all_fields(){
            //Add generic types of fields
            {
                let resolved_field_ty =  field.ty(ctx,subst);
                //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
                if resolved_field_ty != original{
                    res.extend(Self::from_ty(resolved_field_ty,ctx));
                }
            }
            //rustc_middle::ty::List::empty()
            /* 
            let ty = ctx.type_of(field.did).instantiate_identity();
            let ty = Type::from_ty(ty,ctx);
            let name = field.name.to_string().into();
            fields.push((name,ty));*/
        }
        res.push(Self{access,name,inner_types:vec![],fields:vec![],gargc,extends:None});
        res
    }
}
fn escape_type_name(name:IString)->IString{
    if name.as_ref() == "value"{
        "m_value".into()
    }
    else{
        name
    }
}