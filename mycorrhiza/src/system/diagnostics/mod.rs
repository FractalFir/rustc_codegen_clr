pub type Stopwatch = crate::intrinsics::RustcCLRInteropManagedClass<"System.Runtime","System.Diagnostics.Stopwatch ">;
impl Stopwatch{
    #[inline(always)]
    pub fn new()->Self{
        Self::ctor0_()
    }
    #[inline(always)]
    pub fn start(self){
        Self::instance0_::<"Start",()>(self)
    }
    #[inline(always)]
    pub fn stop(self){
        Self::instance0_::<"Stop",()>(self)
    }
    #[inline(always)]
    pub fn reset(self){
        Self::instance0_::<"Reset",()>(self)
    }
    #[inline(always)]
    pub fn restart(self){
        Self::instance0_::<"Restart",()>(self)
    }
    #[inline(always)]
    pub fn elapsed_milliseconds(self)->i64{
        Self::virt0::<"get_ElapsedMilliseconds",i64>(self)
    }
}