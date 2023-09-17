use std::path::Path;
type AssemblyInfo = str;
#[derive(Debug, Clone)]
enum AccessModifer {
    Private,
    Public,
}
use crate::{assembly::Assembly, method::Method, type_def::TypeDef, IString};
pub(crate) mod ilasm_exporter;
pub(crate) trait AssemblyExporter: Sized {
    fn init(asm_info: &AssemblyInfo) -> Self;
    fn add_type(&mut self, tpe: &TypeDef);
    fn add_method(&mut self, method: &Method);
    //fn extern_asm(&mut self,asm:&str);
    fn finalize(self, final_path: &Path) -> Result<(), AssemblyExportError>;
    fn export_assembly(asm: &Assembly, final_path: &Path) -> Result<(), AssemblyExportError> {
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
            .finalize(final_path)
            .expect("Could not export assembly");
        Ok(())
    }
}
#[derive(Debug)]
pub(crate) enum AssemblyExportError {
    InvalidIL,
    CouldNotCanonalizePath(std::io::Error, std::path::PathBuf),
    IoError(std::io::Error),
    ExporterError(IString),
}
impl From<std::io::Error> for AssemblyExportError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
