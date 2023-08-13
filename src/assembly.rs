use crate::IString;
use crate::{CLRMethod, FunctionSignature};
use rustc_middle::{
    mir::mono::MonoItem,
    ty::{Instance, ParamEnv, TyCtxt},
};
pub(crate) struct Assembly {
    methods: Vec<CLRMethod>,
    name: IString,
}
impl Assembly {
    pub(crate) fn into_il_ir(&self) -> IString {
        let mut methods = String::new();
        for method in &self.methods {
            methods.push_str(&method.into_il_ir());
        }
        let methods = format!(".class {name} {{{methods}}}", name = self.name);
        format!(".assembly {name}{{}}\n{methods}", name = self.name).into()
    }
}
impl Assembly {
    pub(crate) fn new(name: &str) -> Self {
        let name: String = name.chars().take_while(|c| *c != '.').collect();
        let name = name.replace('-', "_");
        Self {
            methods: Vec::with_capacity(0x100),
            name: name.into(),
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
        for block_data in blocks {
            clr_method.begin_bb();
            for statement in &block_data.statements {
                clr_method.add_statement(statement, mir, &tcx);
            }
            match &block_data.terminator {
                Some(term) => clr_method.add_terminator(term,mir,&tcx),
                None => (),
            }
        }
        // Optimization is currently broken, and may produce invalid IR.
        //clr_method.opt();
        //clr_method.typecheck();
        clr_method.add_locals(&mir.local_decls);
        println!("clr_method:{clr_method:?}");
        println!("instance:{instance:?}\n");
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
}
