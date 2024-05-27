use std::path::Path;
/// Describes an assemnly.
type AssemblyInfo = str;

use crate::{
    assembly::Assembly,
    config,

    r#type::{Type, TypeDef},
    IString,
};
use cilly::{fn_sig::FnSig, method::Method};
pub mod c_exporter;

/// ILASM-based assembly exporter.
pub mod ilasm_exporter;
mod ilasm_op;
/// This trait represents an interface implemented by all .NET assembly exporters. (Currently only ilasm)
pub trait AssemblyExporter: Sized {
    /// Initializes an assembly exporter.
    fn init(asm_info: &AssemblyInfo) -> Self;
    /// Adds type definition `tpe` to the assembly.
    fn add_type(&mut self, tpe: &TypeDef);
    /// Adds method to assembly.
    fn add_method(&mut self, method: &Method);
    fn add_extern_method(&mut self, lib_path: &str, name: &str, sig: &FnSig);
    //fn extern_asm(&mut self,asm:&str);
    /// Finishes exporting the assembly.
    fn finalize(self, final_path: &Path, is_dll: bool) -> Result<(), AssemblyExportError>;
    /// Adds a reference to assembly `asm_name` with info `info`
    fn add_extern_ref(&mut self, asm_name: &str, info: &crate::assembly::AssemblyExternRef);
    /// Adds a global field
    fn add_global(&mut self, tpe: &Type, name: &str);
    /// Handles the whole assembly export process all at once.
    fn export_assembly(
        asm: &Assembly,
        final_path: &Path,
        is_dll: bool,
    ) -> Result<(), AssemblyExportError> {
        let mut asm_exporter = Self::init("asm");
        for (asm_name, asm_ref) in asm.extern_refs() {
            asm_exporter.add_extern_ref(asm_name, asm_ref);
        }
        for tpe in asm.types() {
            asm_exporter.add_type(tpe.1);
        }
        for method in asm.methods() {
            let mut method = method.clone();
            method.sheed_trees();
            method.allocate_temporaries();
            method.sheed_trees();
            method.allocate_temporaries();
            if *config::ESCAPE_NAMES {
                method.set_name(&escape_class_name(method.name()));
                asm_exporter.add_method(&method);
            } else {
                asm_exporter.add_method(&method);
            }
        }
        for ((name, sig), lib) in asm.extern_fns() {
            asm_exporter.add_extern_method(lib, name, sig);
        }
        for global in asm.globals() {
            asm_exporter.add_global(global.1, global.0);
        }
        /*
        crate::libc::insert_libc(&mut asm_exporter);
        if let Some(entrypoint) = asm.entrypoint() {
            asm_exporter.add_method(crate::codegen::entrypoint::wrapper(entrypoint));
        }*/
        asm_exporter
            .finalize(final_path, is_dll)
            .expect("Could not export assembly");
        Ok(())
    }
}
#[derive(Debug)]
/// Represents an error which happened during assembly exporting.
pub enum AssemblyExportError {
    /// Assemmbly IL was invalid
    InvalidIL,
    /// Could not turn assembly path from relative to absoulte.
    CouldNotCanonalizePath(std::io::Error, std::path::PathBuf),
    /// A generic IO error happended when exporting the assembly.
    IoError(std::io::Error),
    /// The exporter command (ILASM) failed with an error message.
    ExporterError(IString),
}
impl From<std::io::Error> for AssemblyExportError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
#[must_use]
pub fn escape_class_name(name: &str) -> String {
    name.replace("::", ".")
        .replace("..", ".")
        .replace('$', "_dsig_")
        .replace('<', "_lt_")
        .replace('\'', "_ap_")
        .replace(' ', "_spc_")
        .replace('>', "_gt_")
        .replace('(', "_lpar_")
        .replace(')', "_rpar")
        .replace('{', "_lbra_")
        .replace('}', "_rbra")
        .replace('[', "_lsbra_")
        .replace(']', "_rsbra_")
        .replace('+', "_pls_")
        .replace('-', "_hyp_")
        .replace(',', "_com_")
        .replace('*', "_ptr_")
        .replace('#', "_hsh_")
        .replace('&', "_ref_")
        .replace(';', "_scol_")
        .replace('!', "_excl_")
        .replace('\"', "_qt_")
}
