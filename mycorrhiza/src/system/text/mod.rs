pub type StringBuilder =
    crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime", "System.Text.StringBuilder">;
impl StringBuilder {
    #[inline(always)]
    pub fn empty() -> Self {
        Self::ctor0()
    }
    #[inline(always)]
    pub fn append_mchar(self, chr: crate::DotNetChar) -> Self {
        self.instance1::<"Append", crate::DotNetChar, Self>(chr)
    }
    #[inline(always)]
    pub fn append_char(self, chr: char) -> Self {
        self.append_mchar(crate::DotNetChar::single_codepoint_unchecked(chr))
    }
}
