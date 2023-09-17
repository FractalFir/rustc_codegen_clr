#[derive(Debug)]
/// Repersentation of an error which occured while converting MIR to CIL assembly.
pub enum CodegenError {
    UnersolvedGeneric,
}
impl Into<rustc_errors::ErrorGuaranteed> for CodegenError {
    fn into(self) -> rustc_errors::ErrorGuaranteed {
        self.report_error();
        error_guaranteed()
    }
}
impl CodegenError {
    fn report_error(&self) {
        use std::io::Write;
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "CodegenError:{self:?}").expect("Could not report error to stderr");
    }
}
fn error_guaranteed() -> rustc_errors::ErrorGuaranteed {
    unsafe { std::mem::transmute(()) }
}
