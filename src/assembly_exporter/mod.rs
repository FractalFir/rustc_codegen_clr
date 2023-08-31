pub(crate) type GenericArgument = ();
pub(crate) type EnumVariant = ();
pub(crate) type Attribute = ();
pub(crate) type Method = CLRMethod;
use std::path::Path;
type AssemblyInfo = str;
#[derive(Debug, Clone)]
enum AccessModifer {
    Private,
    Public,
}
#[derive(Debug, Clone)]
pub(crate) struct ClassInfo {
    name: IString,
    fields: Vec<(IString, VariableType)>,
    explicit_field_offsets: Option<Vec<u8>>,
    extends: (Option<IString>, IString), //First, optional name of the assembly it comes form, then, type string
    //Optional, can be ignored for now
    access_modifier: AccessModifer,
    member_functions: Vec<Method>,
    generic_args: Vec<GenericArgument>,
    attribute: Vec<Attribute>,
}
impl ClassInfo {
    pub(crate) fn new(name: &str, fields: &[(IString, VariableType)]) -> Self {
        Self {
            name: name.into(),
            fields: fields.into(),
            extends: (Some("System.Runtime".into()), "System.ValueType".into()),
            explicit_field_offsets: None,
            access_modifier: AccessModifer::Public,
            member_functions: vec![],
            generic_args: vec![],
            attribute: vec![],
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn extends(&self) -> &(Option<IString>, IString) {
        &self.extends
    }
    pub(crate) fn fields(&self) -> &[(IString, VariableType)] {
        &self.fields
    }
}
use crate::{assembly::Assembly, clr_method::CLRMethod, IString, VariableType};
pub(crate) mod ilasm_exporter;
pub(crate) trait AssemblyExporter: Sized {
    fn init(asm_info: &AssemblyInfo) -> Self;
    fn add_class(&mut self, class: ClassInfo);
    fn add_method(&mut self, method: CLRMethod);
    fn finalize(self, final_path: &Path) -> Result<(), AssemblyExportError>;
    fn export_assembly(asm: &Assembly, final_path: &Path) -> Result<(), AssemblyExportError> {
        let mut asm_exporter = Self::init(asm.name());
        for struct_type in asm.structs() {
            asm_exporter.add_class(struct_type);
        }
        for method in asm.methods() {
            asm_exporter.add_method(method.clone());
        }
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
