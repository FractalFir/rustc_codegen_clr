use crate::{CLRMethod, FunctionSignature, IString, VariableType};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{mono::MonoItem, Local, LocalDecl},
    ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, Serialize, Deserialize)]
enum Visiblity {
    Private,
    Public,
}
fn array_indexers(array_name: &str, element: &VariableType) -> String {
    //let deref_op = "ldind.i4";
    let element_type = &element.il_name();
    let deref_op = element.deref_op().clr_ir();
    let set_op = element.set_pointed_op().clr_ir();
    let getter = format!(
        "\
    \t.method public hidebysig specialname instance {element_type} get_Item(native int index){{\n\
		\t\tldarg.0\n\
		\t\tldflda {element_type} {array_name}::arr\n\
        \t\tldarg.1\n\
        \t\tsizeof {element_type}\n\
        \t\tmul\n\
        \t\tadd\n\
        \t{deref_op}\
		\t\tret\n\
     }}\n"
    );
    let setter = format!("\
     \t.method public hidebysig specialname instance void set_Item(native int index,{element_type} 'value'){{\n\
		\t\tldarg.0\n\
		\t\tldflda {element_type} {array_name}::arr\n\
        \t\tldarg.1\n\
        \t\tsizeof {element_type}\n\
        \t\tmul\n\
        \t\tadd\n\
        \t\tldarg.2\n\
        \t{set_op}\
		\t\tret\n\
     }}\n");
    format!("{getter}{setter}")
}
#[derive(Clone, Debug, Serialize, Deserialize)]
enum CLRType {
    Struct {
        fields: Vec<(IString, VariableType)>,
    },
    Array {
        element: VariableType,
        length: usize,
    },
    Slice(VariableType),
}
impl CLRType {
    pub(crate) fn get_field_getter(&self, field: usize, field_parent: &str) -> Vec<crate::BaseIR> {
        match self {
            Self::Struct { fields } => {
                let field = &fields[field];
                vec![crate::BaseIR::LDField {
                    field_parent: field_parent.into(),
                    field_name: field.0.clone().into(),
                    field_type: field.1.clone(),
                }]
            }
            Self::Array { .. } | Self::Slice { .. } => {
                panic!("Attempted to get a field of a field-less type!")
            }
        }
    }
    pub(crate) fn get_field_setter(&self, field: usize, field_parent: &str) -> Vec<crate::BaseIR> {
        match self {
            Self::Struct { fields } => {
                let field = &fields[field];
                vec![crate::BaseIR::STField {
                    field_parent: field_parent.into(),
                    field_name: field.0.clone().into(),
                    field_type: field.1.clone(),
                }]
            }
            Self::Array { .. } | Self::Slice { .. } => {
                panic!("Attempted to get a field of a field-less type!")
            }
        }
    }
    pub(crate) fn get_def(&self, name: &str, asm: &Assembly) -> IString {
        match self{
            Self::Struct{fields}=>{
                let mut field_string = String::new();
                for (field_name,field_type) in fields{
                    field_string.push_str(&format!("\t.field public {il_name} {field_name}",il_name = field_type.il_name()));
                }
                format!(".class public sequential {name} extends [System.Runtime]System.ValueType{{\n{field_string}}}\n")
            },
            Self::Array{element,length}=>{
                let indexers = array_indexers(name,element);
            format!(".class public sequential {name} extends [System.Runtime]System.ValueType{{\n\t.pack 0\n\t.size {size}\n\t.field public {element_il} arr\n{indexers}}}\n",element_il= element.il_name(),size = asm.sizeof_type(element)*length)
            },
            Self::Slice(element)=>format!(".class public sequential {name} extends [System.Runtime]System.ValueType{{\n\t.field public {element_il}* ptr\n\t.field public native int cap\n}}\n",element_il= element.il_name()),
        }.into()
    }
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Assembly {
    methods: Vec<CLRMethod>,
    name: IString,
    types: HashMap<IString, CLRType>,
    size_t: u8,
}
impl Assembly {
    pub(crate) fn sizeof_type(&self, var_type: &VariableType) -> usize {
        match var_type {
            VariableType::Void => 0,
            VariableType::I8 | VariableType::U8 | VariableType::Bool => 1,
            VariableType::I16 | VariableType::U16 => 2,
            VariableType::I32 | VariableType::U32 | VariableType::F32 => 4,
            VariableType::I64 | VariableType::U64 | VariableType::F64 => 8,
            VariableType::I128 | VariableType::U128 => 16,
            VariableType::ISize
            | VariableType::USize
            | VariableType::Ref(_)
            | VariableType::RefMut(_)
            | VariableType::Slice(_) => self.size_t as usize,
            VariableType::Array { element, length } => self.sizeof_type(element) * length,
            VariableType::Tuple(elements) => elements
                .iter()
                .map(|element| self.sizeof_type(element))
                .sum::<usize>(),
            VariableType::Struct(struct_name) => {
                match &self.types[struct_name]{
                    CLRType::Struct{fields}=>fields.iter().map(|field|self.sizeof_type(&field.1)).sum::<usize>(),
                    CLRType::Array { element, length } => self.sizeof_type(&element)*length,
                    CLRType::Slice (element) => panic!("Can't compute sizeof silice at compile time!"),
                }
            }
        }
    }
    pub(crate) fn get_field_getter(
        &self,
        field: usize,
        field_parent: &str,
    ) -> Option<Vec<crate::BaseIR>> {
        Some(
            self.types
                .get(field_parent)?
                .get_field_getter(field, field_parent),
        )
    }
    pub(crate) fn get_field_setter(
        &self,
        field: usize,
        field_parent: &str,
    ) -> Option<Vec<crate::BaseIR>> {
        Some(
            self.types
                .get(field_parent)?
                .get_field_setter(field, field_parent),
        )
    }
    //pub(crate) fn (&self,type_name:&str,
    pub(crate) fn into_il_ir(&self) -> IString {
        let mut methods = String::new();
        for method in &self.methods {
            methods.push_str(&method.into_il_ir());
        }
        let mut types = String::new();
        for clr_type in &self.types {
            types.push_str(&clr_type.1.get_def(&clr_type.0.replace('\'', ""), self));
        }
        println!("\nty_count:{}\n", self.types.len());
        //let methods = format!("{methods}");
        format!(".assembly {name}{{}}\n{types}\n{methods}", name = self.name).into()
    }
    pub(crate) fn add_type(&mut self, ty: Ty, tyctx: &TyCtxt) {
        match ty.kind() {
            TyKind::Adt(adt_def, _subst) => {
                // TODO: find a better way to get a name of an ADT!
                let name = format!("{adt_def:?}").into();
                let mut fields = Vec::new();
                for field in adt_def.all_fields() {
                    //TODO: handle binders!
                    fields.push((
                        field.name.to_string().into(),
                        VariableType::from_ty(tyctx.type_of(field.did).skip_binder()),
                    ));
                    println!("field:{field:?}");
                }
                self.types.insert(name, CLRType::Struct { fields });
                println!("adt_def:{adt_def:?} types:{types:?}", types = self.types);
            }
            TyKind::Array(element_type, length) => {
                let (element, length) = (VariableType::from_ty(*element_type), {
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
                let element = VariableType::from_ty(*element_type);
                let name = format!("'RSlice_{element_il}'", element_il = element.il_name()).into();
                let slice = CLRType::Slice(element);
                self.types.insert(name, slice);
            }
            TyKind::Ref(_, ty, _) => self.add_type(*ty, tyctx),
            _ => (),
        }
    }
    pub(crate) fn add_types_from_locals(
        &mut self,
        locals: &IndexVec<Local, LocalDecl>,
        tyctx: &TyCtxt,
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
            types: HashMap::with_capacity(0x100),
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
            FunctionSignature::from_poly_sig(sig)
                .expect("Could not resolve the function signature"),
            name,
        );
        self.add_types_from_locals(&mir.local_decls, &tcx);
        clr_method.add_locals(&mir.local_decls);
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
        self.types.extend(other.types);
    }
}
