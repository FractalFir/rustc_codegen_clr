pub(crate) type GenericType = ();
pub(crate) type EnumVariant = ();
pub(crate) type Method = CLRMethod;
use std::path::Path;
#[derive(Debug, Clone)]
pub(crate) struct StructType {
    name: IString,
    fields: Vec<(IString, VariableType)>,
}
impl StructType {
    pub(crate) fn new(name: &str, fields: &[(IString, VariableType)]) -> Self {
        Self {
            name: name.into(),
            fields: fields.into(),
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn fields(&self) -> &[(IString, VariableType)] {
        &self.fields
    }
    fn has_generics(&self) -> bool {
        self.fields
            .iter()
            .any(|field| matches!(field.1, VariableType::Generic(_)))
    }
}
use crate::{clr_method::CLRMethod, IString, VariableType, assembly::Assembly};
pub(crate) mod ilasm_exporter;
pub(crate) trait AssemblyExporter:Sized {
    fn init(name: &str) -> Self;
    fn add_struct(&mut self, struct_type: StructType);
    fn add_enum_type(
        &mut self,
        name: &str,
        enum_variants: &[EnumVariant],
        generics: &GenericType,
    ) -> Result<(), AssemblyExportError>;
    fn add_array_type(
        &mut self,
        element: &VariableType,
        length: usize,
    ) -> Result<(), AssemblyExportError>;
    fn add_method(&mut self, symbol: &str, method: &Method) -> Result<(), AssemblyExportError>;
    fn finalize(self, final_path: &Path) -> Result<(), AssemblyExportError>;
    fn export_assembly(asm:&Assembly, final_path: &Path) -> Result<(), AssemblyExportError>{
        let mut asm_exporter = Self::init(asm.name());
        for struct_type in asm.structs(){
            asm_exporter.add_struct(struct_type);
        }
        //TODO:methods
        asm_exporter.finalize(final_path)
    }
}
#[derive(Debug)]
pub(crate) enum AssemblyExportError {
    InvalidIL,
    IoError(std::io::Error),
}
impl From<std::io::Error> for AssemblyExportError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
