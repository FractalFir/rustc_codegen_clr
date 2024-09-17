use crate::{
    intrinsics::{RustcCLRInteropManagedClass, RustcCLRInteropManagedStruct},
    FromManagedSafe, IntoManagedSafe, ManagedSafe,
};
type GCHandle = RustcCLRInteropManagedStruct<
    "System.Runtime",
    "System.Runtime.InteropServices.GCHandle",
    { size_of::<usize>() },
>;
pub struct Class<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> {
    handle: GCHandle,
}
impl<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str>
    IntoManagedSafe<RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH>>
    for Class<ASSEMBLY, CLASS_PATH>
{
    fn into_managed(self) -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
        unsafe { self.get_naked_ref() }
    }
}
impl<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> Class<ASSEMBLY, CLASS_PATH> {
    pub type NakedRef = RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH>;
    pub fn ctor0() -> Self {
        Self::from_naked_ref(Self::NakedRef::ctor0())
    }
    pub fn ctor1<Arg: ManagedSafe>(arg: impl IntoManagedSafe<Arg>) -> Self {
        Self::from_naked_ref(Self::NakedRef::ctor1(arg.into_managed()))
    }
    /// Returns the inner reference this handle points to.
    /// # Safety
    /// The type returned by this function is extermaly unsafe, and dealing with it in a sound way is hard.
    /// See [`StackOnly`] trait documentation for more.
    pub unsafe fn get_naked_ref(&self) -> RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH> {
        self.handle.instance0::<"get_Target", Self::NakedRef>()
    }
    pub fn from_naked_ref(naked: RustcCLRInteropManagedClass<ASSEMBLY, CLASS_PATH>) -> Self {
        let object: RustcCLRInteropManagedClass<"System.Runtime", "System.Objetc"> =
            crate::intrinsics::rustc_clr_interop_managed_checked_cast(naked);
        let handle = GCHandle::static1::<"Alloc", _, _>(object);
        Self { handle }
    }
    pub fn instance0<
        const NAME: &'static str,
        SigRet: ManagedSafe,
        RealRet: FromManagedSafe<SigRet>,
    >(
        &mut self,
    ) -> RealRet {
        RealRet::from_managed(unsafe { self.get_naked_ref() }.instance0::<NAME, SigRet>())
    }
    pub fn virt0<
        const NAME: &'static str,
        SigRet: ManagedSafe,
        RealRet: FromManagedSafe<SigRet>,
    >(
        &mut self,
    ) -> RealRet {
        RealRet::from_managed(unsafe { self.get_naked_ref() }.virt0::<NAME, SigRet>())
    }
    pub fn instance1<
        const NAME: &'static str,
        Arg: ManagedSafe,
        SigRet: ManagedSafe,
        RealRet: FromManagedSafe<SigRet>,
    >(
        &mut self,
        arg: impl IntoManagedSafe<Arg>,
    ) -> RealRet {
        RealRet::from_managed(
            unsafe { self.get_naked_ref() }.instance1::<NAME, Arg, SigRet>(arg.into_managed()),
        )
    }
    pub fn instance2<
        const NAME: &'static str,
        Arg: ManagedSafe,
        Arg2: ManagedSafe,
        SigRet: ManagedSafe,
        RealRet: FromManagedSafe<SigRet>,
    >(
        &mut self,
        arg: impl IntoManagedSafe<Arg>,
        arg2: impl IntoManagedSafe<Arg2>,
    ) -> RealRet {
        RealRet::from_managed(
            unsafe { self.get_naked_ref() }
                .instance2::<NAME, Arg, Arg2, SigRet>(arg.into_managed(), arg2.into_managed()),
        )
    }
    //pub fn to_mstring(&self)->
}
impl<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> Drop
    for Class<ASSEMBLY, CLASS_PATH>
{
    fn drop(&mut self) {
        self.handle.instance0::<"Free", ()>()
    }
}
impl<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> Clone
    for Class<ASSEMBLY, CLASS_PATH>
{
    fn clone(&self) -> Self {
        Self::from_naked_ref(unsafe { self.get_naked_ref() })
    }
}
