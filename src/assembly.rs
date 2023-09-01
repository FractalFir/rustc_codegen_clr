use crate::{assembly_exporter::Method, types::Type, CLRMethod, FunctionSignature, IString};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{mono::MonoItem, Local, LocalDecl},
    ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
pub(crate) struct Assembly {
    methods: Vec<CLRMethod>,
    name: IString,
    //types: HashMap<IString, CLRType>,
    size_t: u8,
}
impl Assembly {
    /*
    pub(crate) fn structs(&self) -> Vec<crate::assembly_exporter::ClassInfo> {
        self.types
            .iter()
            .map(|tpe| {
                if let CLRType::Struct { fields } = tpe.1 {
                    Some(crate::assembly_exporter::ClassInfo::new(tpe.0, fields))
                } else {
                    None
                }
            })
            .filter(|strct| strct.is_some())
            .map(|strct| strct.unwrap())
            .collect()
    }*/
    pub(crate) fn methods(&self) -> &[CLRMethod] {
        &self.methods
    }
    pub(crate) fn sizeof_type(&self, var_type: &Type) -> usize {
        match var_type {
            Type::Void => 0,
            Type::I8 | Type::U8 | Type::Bool => 1,
            Type::I16 | Type::U16 => 2,
            Type::I32 | Type::U32 | Type::F32 => 4,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::I128 | Type::U128 => 16,
            Type::ISize | Type::USize | Type::Ref(_) | Type::Ptr(_) => self.size_t as usize,
            Type::Slice(_) => (self.size_t + self.size_t) as usize,
            Type::Array { element, length } => todo!("Can't get sizeof array yet!"), //self.sizeof_type(element) * length,
            Type::Tuple(elements) => elements
                .iter()
                .map(|element| self.sizeof_type(element))
                .sum::<usize>(),
            Type::Struct { name, .. } => todo!("can't take sizeof struct yet!"),
            Type::StrSlice => panic!("Can't compute sizeof string silice at compile time!"),
            //Type::Generic(_) => todo!("Can't calcuate the size of a geneic!"),
            _ => todo!("Cant estimate size of {var_type:?} yet."),
        }
    }
    pub(crate) fn get_field_getter(
        &self,
        field: usize,
        field_parent: &str,
    ) -> Option<Vec<crate::BaseIR>> {
        todo!("Can't get field yet!")
    }
    pub(crate) fn get_field_setter(
        &self,
        field: usize,
        field_parent: &str,
    ) -> Option<Vec<crate::BaseIR>> {
        todo!("Can't set field yet!")
    }
    pub(crate) fn add_type<'ctx>(&mut self, ty: Ty<'ctx>, tyctx: &TyCtxt<'ctx>) {
        /*
        match ty.kind() {
            TyKind::Adt(adt_def, _subst) => {
                // TODO: find a better way to get a name of an ADT!
                let name = format!("{adt_def:?}").replace("::", ".").into();
                let mut fields = Vec::new();
                for field in adt_def.all_fields() {
                    //TODO: handle binders!
                    fields.push((
                        field.name.to_string().into(),
                        Type::from_ty(&tyctx.type_of(field.did).skip_binder(), tyctx),
                    ));
                    println!("field:{field:?}");
                }
                self.types.insert(name, CLRType::Struct { fields });
                println!("adt_def:{adt_def:?} types:{types:?}", types = self.types);
            }
            TyKind::Array(element_type, length) => {
                let (element, length) = (Type::from_ty(*element_type, *tyctx), {
                    let scalar = length
                        .try_to_scalar()
                        .expect("Could not convert the scalar");
                    let value = scalar.to_u64().expect("Could not convert scalar to u64!");
                    value as usize
                });
                let name = format!(
                    "'RArray_{element_il}_{length}'",
                    element_il = element.il_name()
                )
                .into();
                let arr = CLRType::Array { element, length };
                self.types.insert(name, arr);
            }
            TyKind::Slice(element_type) => {
                let element = Type::from_ty(*element_type, *tyctx);
                let name = format!("'RSlice_{element_il}'", element_il = element.il_name()).into();
                let slice = CLRType::Slice(element);
                self.types.insert(name, slice);
            }
            TyKind::Ref(_, ty, _) => self.add_type(*ty, tyctx),
            _ => (),
        }*/
    }
    pub(crate) fn add_types_from_locals<'ctx>(
        &mut self,
        locals: &IndexVec<Local, LocalDecl<'ctx>>,
        tyctx: &TyCtxt<'ctx>,
    ) {
        for local in locals.iter() {
            self.add_type(local.ty, tyctx);
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn new(name: &str) -> Self {
        let name: String = name.chars().take_while(|c| *c != '.').collect();
        let name = name.replace('-', "_");
        Self {
            methods: Vec::with_capacity(0x100),
            //types: HashMap::with_capacity(0x100),
            name: name.into(),
            size_t: 8,
        }
    }
    pub(crate) fn add_fn<'tcx>(&mut self, instance: Instance<'tcx>, tcx: TyCtxt<'tcx>, name: &str) {
        // TODO: figure out: What should it be???
        let param_env = ParamEnv::empty();

        let def_id = instance.def_id();
        let mir = tcx.optimized_mir(def_id);
        let blocks = &(*mir.basic_blocks);
        let sig = instance.ty(tcx, param_env).fn_sig(tcx);
        let mut clr_method = CLRMethod::new(
            FunctionSignature::from_poly_sig(sig, tcx)
                .expect("Could not resolve the function signature"),
            name,
        );
        self.add_types_from_locals(&mir.local_decls, &tcx);
        clr_method.add_locals(&mir.local_decls, tcx);
        for block_data in blocks {
            clr_method.begin_bb();
            for statement in &block_data.statements {
                clr_method.add_statement(statement, mir, tcx, self);
            }
            match &block_data.terminator {
                Some(term) => clr_method.add_terminator(term, mir, &tcx, self),
                None => (),
            }
        }
        clr_method.remove_void_locals();
        clr_method.opt();
        //println!("clr_method:{clr_method:?}");
        //println!("instance:{instance:?}\n");
        //println!("types:{types:?}", types = self.types);
        self.methods.push(clr_method);
    }
    pub(crate) fn add_item<'tcx>(&mut self, item: MonoItem<'tcx>, tcx: TyCtxt<'tcx>) {
        println!("adding item:{}", item.symbol_name(tcx));

        match item {
            MonoItem::Fn(instance) => {
                self.add_fn(instance, tcx, &format!("{}", item.symbol_name(tcx)))
            }
            _ => todo!("Unsupported item:\"{item:?}\"!"),
        }
    }
    pub(crate) fn link(&mut self, other: Self) {
        //TODO: do linking.
        self.methods.extend_from_slice(&other.methods);
        //self.types.extend(other.types);
    }
}
