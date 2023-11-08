use std::path::Path;
type AssemblyInfo = str;
#[derive(Debug, Clone)]
enum AccessModifer {
    Private,
    Public,
}
use crate::{assembly::Assembly, method::Method, type_def::TypeDef, IString};
/// ILASM-based assembly exporter.
pub mod ilasm_exporter;
/// This trait represents an interface implemented by all .NET assembly exporters. (Currently only ilasm)
pub trait AssemblyExporter: Sized {
    /// Initializes an assembly exporter.
    fn init(asm_info: &AssemblyInfo) -> Self;
    /// Adds type definition `tpe` to the assembly.
    fn add_type(&mut self, tpe: &TypeDef);
    /// Adds method to assembly.
    fn add_method(&mut self, method: &Method);
    //fn extern_asm(&mut self,asm:&str);
    /// Finishes exporting the assembly.
    fn finalize(self, final_path: &Path, is_dll: bool) -> Result<(), AssemblyExportError>;
    /// Handles the whole assembly export process all at once.
    fn export_assembly(
        asm: &Assembly,
        final_path: &Path,
        is_dll: bool,
    ) -> Result<(), AssemblyExportError> {
        let mut asm_exporter = Self::init("asm");
        for tpe in asm.types() {
            asm_exporter.add_type(tpe);
        }
        for method in asm.methods() {
            asm_exporter.add_method(method);
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
