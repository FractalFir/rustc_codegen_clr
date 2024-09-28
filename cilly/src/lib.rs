#![allow(clippy::module_name_repetitions)]
#![feature(iter_intersperse, pattern)]

pub use crate::v2::Type;
use fxhash::FxHasher;
use std::collections::HashMap;

pub type IString = Box<str>;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AsmString(u64);
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct AsmStringContainer {
    map: HashMap<AsmString, IString>,
    inv_map: HashMap<IString, AsmString>,
}
impl AsmStringContainer {
    pub fn alloc(&mut self, val: impl Into<IString> + Clone) -> AsmString {
        match self.inv_map.entry(val.clone().into()) {
            std::collections::hash_map::Entry::Occupied(occupied) => *occupied.get(),
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let vstr = val.into();
                let hash = calculate_hash(&vstr);
                let string = AsmString(hash);
                assert!(self.map.insert(string, vstr.clone()).is_none());
                *vacant.insert(string)
            }
        }
    }
    pub fn join(&mut self, other: &mut Self) {
        self.map.extend(other.map.clone());
        self.inv_map.extend(other.inv_map.clone());
    }
    pub fn get(&self, key: AsmString) -> &IString {
        &self.map[&key]
    }
}
pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}

pub mod access_modifier;
pub mod asm;

pub mod basic_block;

pub mod call_site;
pub mod cil_iter;
pub mod cil_iter_mut;
pub mod cil_node;
pub mod cil_root;
pub mod cil_tree;
pub mod entrypoint;

pub mod libc_fns;
pub mod method;

pub mod utilis;
pub mod v2;

#[must_use]
pub fn mem_checks() -> bool {
    *crate::MEM_CHECKS
}
#[must_use]
pub fn debig_sfi() -> bool {
    *crate::DEBUG_SFI
}
use lazy_static::lazy_static;
lazy_static! {
    #[doc = "Tells codegen to insert memory consistency checks after each call. If INSERT_MIR_DEBUG_COMMENTS is enabled, the consistency checks will be run also after each MIR statement."]pub static ref MEM_CHECKS:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(MEM_CHECKS){
            Some(value)
        }else {
            None
        }).is_some_and(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(MEM_CHECKS),value),
        })
    };
}

pub fn sfi_debug_print(sfi: &crate::cil_root::SFI) -> String {
    format!(
        "ldstr {name:?}
        call void [System.Console]System.Console::Write(string)
        ldstr \": \"
        call void [System.Console]System.Console::Write(string)
        ldc.i4 {ls}
        call void [System.Console]System.Console::Write(uint32)
        ldstr \"..\"
        call void [System.Console]System.Console::Write(string)
        ldc.i4 {le}
        call void [System.Console]System.Console::WriteLine(uint32)
        call class [System.Runtime]System.Reflection.MethodBase [System.Runtime]System.Reflection.MethodBase::GetCurrentMethod()
        callvirt instance string [System.Runtime]System.Reflection.MemberInfo::get_Name()
        call void [System.Console]System.Console::WriteLine(string)
        ",
        name = sfi.2,
        ls = sfi.0.start,
        le = sfi.0.start,
       // col = sfi.1,
    )
}

lazy_static! {
    #[doc = "Tells codegen to display source file info when executing each statement. "]pub static ref DEBUG_SFI:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(DEBUG_SFI){
            Some(value)
        }else {
            None
        }).is_some_and(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(DEBUG_SFI),value),
        })
    };
}

#[derive(Copy, Clone)]
pub struct DepthSetting(u32);
impl DepthSetting {
    pub fn with_pading() -> Self {
        Self(0)
    }
    pub fn no_pading() -> Self {
        Self(u32::MAX)
    }
    pub fn pad(&self, out: &mut impl std::fmt::Write) -> std::fmt::Result {
        writeln!(out)?;
        if self.0 == u32::MAX {
            return Ok(());
        }
        for _ in 0..self.0 {
            write!(out, " ")?;
        }
        Ok(())
    }
    pub fn incremented(self) -> Self {
        if self.0 == u32::MAX {
            self
        } else {
            Self(self.0 + 1)
        }
    }
}

pub fn escape_type_name(name: &str) -> String {
    name.replace(['.', ' '], "_")
        .replace('<', "lt")
        .replace('>', "gt")
        .replace('$', "ds")
        .replace(',', "cm")
        .replace('{', "bs")
        .replace('}', "be")
        .replace('+', "ps")
}
#[macro_export]
macro_rules! source_info {
    () => {
        CILRoot::source_info(
            file!(),
            (line!() as u64)..(line!() as u64),
            (column!() as u64)..(column!() as u64 + 1),
        )
        .into()
    };
}
