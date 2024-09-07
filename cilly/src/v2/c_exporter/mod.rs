// This exporter is WIP.
#![allow(dead_code, unused_imports, unused_variables, clippy::let_unit_value)]
use std::{collections::HashSet, io::Write};

use fxhash::FxHashSet;

use crate::{utilis::assert_unique, v2::MethodImpl};

use super::{
    cilroot::BranchCond, int, Assembly, CILIter, CILIterElem, CILNode, ClassDefIdx, ClassRefIdx,
    Exporter, NodeIdx, RootIdx, Type,
};

pub struct CExporter {
    is_lib: bool,
}
impl CExporter {
    #[must_use]
    pub fn new(is_lib: bool) -> Self {
        Self { is_lib }
    }
    #[allow(clippy::too_many_arguments)]
    fn export_class(
        &self,
        asm: &super::Assembly,
        defid: ClassDefIdx,
        method_decls: &mut impl Write,
        method_defs: &mut impl Write,
        type_defs: &mut impl Write,
        defined_types: &mut FxHashSet<ClassDefIdx>,
        delayed_defs: &mut FxHashSet<ClassDefIdx>,
    ) -> std::io::Result<()> {
        todo!();
    }
    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        let mut method_defs = Vec::new();
        let mut method_decls = Vec::new();
        let mut type_defs = Vec::new();
        let mut defined_types: FxHashSet<ClassDefIdx> = FxHashSet::default();
        let mut delayed_defs: FxHashSet<ClassDefIdx> = FxHashSet::default();
        for class_def in asm.iter_class_def_ids() {
            self.export_class(
                asm,
                *class_def,
                &mut method_decls,
                &mut method_defs,
                &mut type_defs,
                &mut defined_types,
                &mut delayed_defs,
            )?;
        }
        while !delayed_defs.is_empty() {
            /*
            self.export_class(
                asm,
                *class_def,
                &mut method_decls,
                &mut method_defs,
                &mut type_defs,
                &mut defined_types,
                &mut delayed_defs,
            )?; */
            todo!();
        }
        //
        if true {
            todo!();
        }

        Ok(())
    }
}

impl Exporter for CExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let c_path = target.with_extension("il");
        let mut c_out = std::io::BufWriter::new(std::fs::File::create(&c_path)?);
        self.export_to_write(asm, &mut c_out)?;
        // Needed to ensure the IL file is valid!
        c_out.flush().unwrap();
        drop(c_out);
        let exe_out = target;

        let mut cmd = std::process::Command::new(std::env::var("CC").unwrap_or("cc".to_owned()));
        cmd.arg(c_path)
        .arg("-o")
        .arg(exe_out)
        .arg("-g")
        .arg("-Ofast")
        // .arg("-FOLD") saves up on space, consider enabling.
        ;
        let out = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !(stderr.contains("error") || stderr.contains("fatal")),
            "stdout:{} stderr:{} cmd:{cmd:?}",
            stdout,
            String::from_utf8_lossy(&out.stderr)
        );

        Ok(())
    }
}

#[must_use]
pub fn class_to_mangled(class: &super::ClassRef, asm: &Assembly) -> String {
    let assembly = match class.asm() {
        Some(asm_idx) => asm.get_string(asm_idx).as_ref(),
        None => "",
    };
    format!("{assembly}{name}", name = asm.get_string(class.name()))
}
#[must_use]
pub fn name_sig_class_to_mangled(
    name: &str,
    sig: super::SigIdx,
    class: Option<ClassRefIdx>,
    asm: &Assembly,
) -> String {
    let class = match class {
        Some(_) => todo!(),
        None => todo!(),
    };
}
