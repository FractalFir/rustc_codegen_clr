#[derive(Clone,Copy)]
#[repr(C)]
pub struct RustcCLRInteropManagedClass<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str>{
    size_hint:usize,
}
impl<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str> RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    #[inline(always)]
    pub fn ctor0_()->Self{
        rustc_clr_interop_managed_ctor0_::<ASSEMBLY,CLASS_PATH,false>()
    }
    #[inline(always)]
    pub fn ctor1_<Arg1>(arg1:Arg1)->Self{
        rustc_clr_interop_managed_ctor1_::<ASSEMBLY,CLASS_PATH,false,Arg1>(arg1)
    }
    #[inline(always)]
    pub fn ctor2_<Arg1,Arg2>(arg1:Arg1,arg2:Arg2)->Self{
        rustc_clr_interop_managed_ctor2_::<ASSEMBLY,CLASS_PATH,false,Arg1,Arg2>(arg1,arg2)
    }
    #[inline(always)]
    pub fn ctor3_<Arg1,Arg2,Arg3>(arg1:Arg1,arg2:Arg2,arg3:Arg3)->Self{
        rustc_clr_interop_managed_ctor3_::<ASSEMBLY,CLASS_PATH,false,Arg1,Arg2,Arg3>(arg1,arg2,arg3)
    }
    #[inline(always)]
    pub fn static0<const METHOD:&'static str,Ret>()->Ret{
        rustc_clr_interop_managed_call0_::<ASSEMBLY,CLASS_PATH,false,METHOD,Ret>()
    }
    #[inline(always)]
    pub fn instance0_<const METHOD:&'static str,Ret>(self)->Ret{
        rustc_clr_interop_managed_call1_::<ASSEMBLY,CLASS_PATH,false,METHOD,false,Ret,Self>(self)
    }
    #[inline(always)]
    pub fn virt0<const METHOD:&'static str,Ret>(self)->Ret{
        rustc_clr_interop_managed_call_virt1_::<ASSEMBLY,CLASS_PATH,false,METHOD,false,Ret,Self>(self)
    }
    #[inline(always)]
    pub fn static1<const METHOD:&'static str,Arg1,Ret>(arg1:Arg1)->Ret{
        rustc_clr_interop_managed_call1_::<ASSEMBLY,CLASS_PATH,false,METHOD,true,Ret,Arg1>(arg1)
    }
    #[inline(always)]
    pub fn static2<const METHOD:&'static str,Arg1,Arg2,Ret>(arg1:Arg1,arg2:Arg2)->Ret{
        rustc_clr_interop_managed_call2_::<ASSEMBLY,CLASS_PATH,false,METHOD,true,Ret,Arg1,Arg2>(arg1,arg2)
    }
    #[inline(always)]
    pub fn instance1_<const METHOD:&'static str,Arg1,Ret>(self,arg1:Arg1)->Ret{
        rustc_clr_interop_managed_call2_::<ASSEMBLY,CLASS_PATH,false,METHOD,false,Ret,Self,Arg1>(self,arg1)
    }
    #[inline(always)]
    pub fn to_mstring(self)->crate::system::MString{
        self.instance0_::<"ToString",crate::system::MString>()
    }
}
#[derive(Clone,Copy)]
#[repr(C)]
pub struct RustcCLRInteropManagedChar{
    utf16_char:u16,
}
#[derive(Clone,Copy)]
#[repr(C)]
struct RustcCLRInteropManagedArray<T,const DIMENSIONS:usize>{
    object_ref:usize,
    pd:core::marker::PhantomData<T>,
}
//Calls
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,Ret>()->Ret{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1>(arg1:Arg1)->Ret{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call2_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1,Arg2>(arg1:Arg1,arg2:Arg2)->Ret{
    core::intrinsics::abort();
}
//VCalls
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call_virt0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,Ret>()->Ret{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call_virt1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1>(arg1:Arg1)->Ret{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_call_virt2_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1,Arg2>(arg1:Arg1,arg2:Arg2)->Ret{
    core::intrinsics::abort();
}
//Ctors
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_ctor0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool>()->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_ctor1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1>(arg1:Arg1)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_ctor2_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1,Arg2>(arg1:Arg1,arg2:Arg2)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
#[allow(unused_variables)]
#[inline(never)]
pub fn rustc_clr_interop_managed_ctor3_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,Arg1,Arg2,Arg3>(arg1:Arg1,arg2:Arg2,arg3:Arg3)->RustcCLRInteropManagedClass<ASSEMBLY,CLASS_PATH>{
    core::intrinsics::abort();
}
impl From<u16> for RustcCLRInteropManagedChar{
    fn from(utf16_char:u16)->RustcCLRInteropManagedChar{
        unsafe{core::mem::transmute::<u16,RustcCLRInteropManagedChar>(core::intrinsics::black_box(utf16_char))}
    }
}
/*
impl TryFrom<char> for RustcCLRInteropManagedChar{
    type Error = (RustcCLRInteropManagedChar,RustcCLRInteropManagedChar);
    fn try_from(value: char) -> Result<RustcCLRInteropManagedChar, (RustcCLRInteropManagedChar,RustcCLRInteropManagedChar)> {
        let byte1 = (value as u64) & 0xFF;
        if (byte1 & 0x80) == 0x00{
            //1 byte long char
            let utf16 = (byte1 & 0x7F) as u16;
            Ok(utf16.into())
        }
        else if (byte1 & 0xE0) == 0xC0{
            //2 byte long char
            let byte2 = ((value as u64) & 0x00FF)>>8;
            let utf16 = (((byte1 & 0x1F)<<6) | (byte2 & 0x3F)) as u16;
            Ok(utf16.into())
        }
        else if (byte1 & 0xF0) == 0xE0{
            //3 byte long char
            let byte2 = ((value as u64) & 0x00FF)>>8;
            let byte3 = ((value as u64) & 0x0000FF)>>16;
            let utf16  = (((byte1 & 0x0F)<<12)
            | ((byte2 & 0x3F) << 6)
            | (byte3 & 0x3F)) as u16;
            Ok(utf16.into())
        }
        else if (byte1 & 0xF8) == 0xF0{
            let byte2 = ((value as u64) & 0x00FF)>>8;
            let byte3 = ((value as u64) & 0x0000FF)>>16;
            let byte4 = ((value as u64) & 0x000000FF)>>24;
                  //4 byte long char
            let code_point = ((byte1 & 0x07) << 18) | ((byte2 & 0x3F)<< 12) | ((byte3 & 0x3F)<< 6) | (byte4 & 0x3F);
            let code_point = code_point - 0x10000;
            let high_surrogate = (0xD800 + ((code_point >> 10) & 0x3FF)) as u16;
            let low_surrogate = (0xDC00 + (code_point & 0x3FF)) as u16;
            Err((high_surrogate.into(), low_surrogate.into()))
        }
        else{
            //Invalid char.
            Ok(0xFFFD.into())
        }
    }
}*/
impl RustcCLRInteropManagedChar{
    pub fn single_codepoint_unchecked(value: char)->Self{
        let byte1 = (value as u64) & 0xFF;
        if (byte1 & 0x80) == 0x00{
            //1 byte long char
            let utf16 = (byte1 & 0x7F) as u16;
            utf16.into()
        }
        else if (byte1 & 0xE0) == 0xC0{
            //2 byte long char
            let byte2 = ((value as u64) & 0x00FF)>>8;
            let utf16 = (((byte1 & 0x1F)<<6) | (byte2 & 0x3F)) as u16;
            utf16.into()
        }
        else if (byte1 & 0xF0) == 0xE0{
            //3 byte long char
            let byte2 = ((value as u64) & 0x00FF)>>8;
            let byte3 = ((value as u64) & 0x0000FF)>>16;
            let utf16  = (((byte1 & 0x0F)<<12)
            | ((byte2 & 0x3F) << 6)
            | (byte3 & 0x3F)) as u16;
            utf16.into()
        }
        else if (byte1 & 0xF8) == 0xF0{
            //4 byte long char
            0xFFFD.into()
        }
        else{
            //Invalid utf8.
            0xFFFD.into()
        }
    }
}