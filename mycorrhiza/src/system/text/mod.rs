pub type StringBuilder = crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime","System.Text.StringBuilder">;
impl StringBuilder{
    #[inline(always)]
    pub fn empty()->Self{
        Self::ctor0_()
    }
    #[inline(always)]
    pub fn append_mchar(self,chr:crate::DotNetChar)->Self{
        self.instance1_::<"Append",crate::DotNetChar,Self>(chr)
    }
    #[inline(always)]
    pub fn append_char(self,chr:char)->Self{
        match chr.try_into(){
            Ok(chr)=>self.append_mchar(chr),
            Err((chr1,chr2))=>self.append_mchar(chr1).append_mchar(chr2),
        }
    }
}