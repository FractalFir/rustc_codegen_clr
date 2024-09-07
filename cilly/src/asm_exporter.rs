pub type AssemblyInfo = str;

use std::path::Path;

use crate::{
    asm::{Assembly, AssemblyExternRef},
    fn_sig::FnSig,
    method::Method,
    type_def::ClassDef,
    IString, Type,
};

/// This trait represents an interface implemented by all .NET assembly exporters. (Currently only ilasm)
pub trait AssemblyExporter: Sized {
    /// Adds type definition `tpe` to the assembly.
    fn add_type(&mut self, tpe: &ClassDef, asm: &Assembly);
    /// Adds method to assembly.
    fn add_method(&mut self, method: &Method, asm: &Assembly);
    fn add_extern_method(
        &mut self,
        lib_path: &str,
        name: &str,
        sig: &FnSig,
        preserve_errno: bool,
        info: &Assembly,
    );
    //fn extern_asm(&mut self,asm:&str);
    /// Finishes exporting the assembly.
    fn finalize(self, final_path: &Path, is_dll: bool) -> Result<(), AssemblyExportError>;
    /// Adds a reference to assembly `asm_name` with info `info`
    fn add_extern_ref(&mut self, asm_name: &str, info: &AssemblyExternRef);
    /// Adds a global field
    fn add_global(&mut self, tpe: &Type, name: &str, thread_local: bool, info: &Assembly);
    /// Handles the whole assembly export process all at once.
    fn export_assembly(
        mut self,
        asm: &Assembly,
        final_path: &Path,
        is_dll: bool,
        escape_names: bool,
    ) -> Result<(), AssemblyExportError> {
        for (asm_name, asm_ref) in asm.extern_refs() {
            self.add_extern_ref(asm_name, asm_ref);
        }
        for tpe in asm.types() {
            self.add_type(tpe.1, asm);
        }
        for method in asm.methods() {
            if escape_names {
                let mut method = method.clone();
                method.set_name(&escape_class_name(method.name()));
                self.add_method(&method, asm);
            } else {
                self.add_method(method, asm);
            }
        }
        for ((name, sig, preserve_errno), lib) in asm.extern_fns() {
            self.add_extern_method(lib, name, sig, *preserve_errno, asm);
        }
        for global in asm.globals() {
            self.add_global(&global.1 .0, global.0, global.1 .1, asm);
        }

        self.finalize(final_path, is_dll)
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
    /// A generic formatter error happended when exporting the assembly.
    FmtError(std::fmt::Error),
}
impl From<std::io::Error> for AssemblyExportError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
impl From<std::fmt::Error> for AssemblyExportError {
    fn from(error: std::fmt::Error) -> Self {
        Self::FmtError(error)
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
#[test]
fn escaped_left_alone() {
    let escaped = "_ZN70_dsLTdscore__option__OptiondsLTdsTdsGTdsdsu20dsasdsu20dscore__cmp__PartialEqdsGTds2eq17h63ae1ba4b0f01183E";
    assert_eq!(escape_class_name(escaped), escaped);
}
