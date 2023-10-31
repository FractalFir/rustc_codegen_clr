use std::fmt::Debug;

#[derive(Debug)]
/// Repersentation of an error which occured while converting MIR to CIL assembly.
pub enum CodegenError {
    UnersolvedGeneric,
    Error(crate::IString),
    Method(MethodCodegenError),
}
impl From<CodegenError> for rustc_errors::ErrorGuaranteed {
    fn from(val: CodegenError) -> Self {
        val.report_error();
        error_guaranteed()
    }
}
impl From<MethodCodegenError> for CodegenError{
    fn from(value: MethodCodegenError) -> Self {
        Self::Method(value)
    }
} 
impl CodegenError {
    fn report_error(&self) {
        eprintln!("Codegen faliled with error:{self:?}")
    }
    pub fn from_panic_message(msg:&str)->Self{
        Self::Error(msg.into())
    }
}
fn error_guaranteed() -> rustc_errors::ErrorGuaranteed {
    unsafe { std::mem::transmute(()) }
}
pub struct MethodCodegenError {
    file: String,
    line: u32,
    column: u32,
    message: String,
}
impl MethodCodegenError {
    pub fn new(file: &str, line: u32, column: u32, message: String) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            message,
        }
    }
    pub fn report(&self) {
        eprintln!(
            "Method Codegen Error: {file}({line},{column}): {message}",
            file = self.file,
            line = self.line,
            column = self.column,
            message = self.message
        );
    }
}
impl Debug for MethodCodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Method Codegen Error: {file}({line},{column}): {message}",
            file = self.file,
            line = self.line,
            column = self.column,
            message = self.message
        )
    }
}
/// Modifies the behavoiour of the `codegen_error` macro, making it panic if true and report the error if false.
pub const PANIC_ON_ERROR: bool = false;
#[macro_export]
macro_rules! codegen_error {
    () => {
        let mce = $crate::codegen_error::MethodCodegenError::new(file!(),line!(),column!(),format!());
        if crate::codegen_error::PANIC_ON_ERROR{
            panic!()
        }
        return Err(mce).into();
    };
    ($($arg:tt)+) => {{
        let mce = crate::codegen_error::MethodCodegenError::new(file!(),line!(),column!(),format_args!($($arg)+).to_string());
        if crate::codegen_error::PANIC_ON_ERROR{
            panic!($($arg)+)
        }
        return Err(mce).into();
    }
    };
}
