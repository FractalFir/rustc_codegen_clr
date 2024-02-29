#![allow(non_upper_case_globals, dead_code)]
use core::num::NonZeroI32;
use lazy_static::*;

use crate::r#type::Type;

use super::AssemblyExporter;
#[repr(C)]
#[derive(Default)]
struct Version {
    major: u16,
    minor: u16,
    build: u16,
    revision: u16,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct AsmBuilderHandle(NonZeroI32);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct FieldDefHandle(NonZeroI32);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct TypeHandle(NonZeroI32);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct TypeDefHandle(NonZeroI32);
#[repr(C)]
struct RustStr<'a> {
    ptr: *const u8,
    len: usize,
    pd: std::marker::PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Default)]
struct TypeAttributes(u16);
impl TypeAttributes {
    fn sealed(self) -> Self {
        Self(self.0 ^ 256)
    }
    fn public(self) -> Self {
        Self(self.0 ^ 1)
    }
    fn explicit(self) -> Self {
        Self(self.0 ^ 16)
    }
}
impl<'a> From<&'a str> for RustStr<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            ptr: s.as_ptr(),
            len: s.len(),
            pd: std::marker::PhantomData,
        }
    }
}

lazy_static! {
    static ref ASSEMBLY_UTILIS: libloading::Library = unsafe {
        libloading::Library::new("/home/michal/Rust/rustc_codegen_clr/AssemblyUtilis/bin/Release/net8.0/linux-x64/native/AssemblyUtilis.so").expect("Could not load the library 'AssemblyUtilis'.")
    };
    static ref new_assembly_def: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(
            asm_name: RustStr,
            mod_name: RustStr,
            is_lib: bool,
            version: Version,
        ) -> Option<AsmBuilderHandle>,
    > = unsafe {
        ASSEMBLY_UTILIS
            .get(b"new_assembly_def")
            .expect("Can't find function 'new_assembly_def'.")
    };
    static ref new_type_def: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(
            namespace: RustStr,
            name: RustStr,
            attr: TypeAttributes,
        ) -> Option<TypeDefHandle>,
    > = unsafe {
        ASSEMBLY_UTILIS
            .get(b"new_type_def")
            .expect("Can't find function 'new_type_def'.")
    };
    static ref add_typedef: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>, tpe: Option<TypeDefHandle>),
    > = unsafe {
        ASSEMBLY_UTILIS
            .get(b"add_typedef")
            .expect("Can't find function 'add_typedef'.")
    };
    static ref new_field_def: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(
            field_name: RustStr,
            field_attrs: TypeAttributes,
            field_type: TypeHandle,
            has_offset: bool,
            offset: u32,
        ) -> FieldDefHandle,
    > = unsafe {
        ASSEMBLY_UTILIS
            .get(b"new_field_def")
            .expect("Can't find function 'new_field_def'.")
    };
    static ref serialize_assembly: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>, path: RustStr),
    > = unsafe { ASSEMBLY_UTILIS.get(b"serialize_assembly").unwrap() };
    static ref int8: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"int8").unwrap() };
    static ref int16: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"int16").unwrap() };
    static ref int32: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"int32").unwrap() };
    static ref uint8: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"uint8").unwrap() };
    static ref uint16: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"uint16").unwrap() };
    static ref uint32: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"uint32").unwrap() };
    static ref nuint: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"nuint").unwrap() };
    static ref void: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"void").unwrap() };
    static ref get_bool: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"bool").unwrap() };
    static ref valuetype: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(asm: Option<AsmBuilderHandle>) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"valuetype").unwrap() };
    static ref type_ref_to_pointer: libloading::Symbol<'static, unsafe extern "cdecl" fn(tpe: TypeHandle) -> TypeHandle> =
        unsafe { ASSEMBLY_UTILIS.get(b"type_ref_to_pointer").unwrap() };
    static ref add_field_def: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(typedef: TypeDefHandle, field: FieldDefHandle) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"add_field_def").unwrap() };
    static ref new_type_ref: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(
            namespace: RustStr,
            name: RustStr,
            is_valuetype: bool,
        ) -> TypeHandle,
    > = unsafe { ASSEMBLY_UTILIS.get(b"new_type_ref").unwrap() };
    static ref add_inner_type: libloading::Symbol<
        'static,
        unsafe extern "cdecl" fn(owner: TypeDefHandle, inner: TypeDefHandle),
    > = unsafe { ASSEMBLY_UTILIS.get(b"add_inner_type").unwrap() };
    static ref set_typedef_baseclass: libloading::Symbol<'static, unsafe extern "cdecl" fn(tpe: TypeDefHandle, baseClass: TypeHandle)> =
        unsafe { ASSEMBLY_UTILIS.get(b"set_typedef_baseclass").unwrap() };
}
impl TypeDefHandle {
    pub fn new(namespace: &str, class_name: &str, attrs: TypeAttributes) -> Self {
        unsafe {
            new_type_def(namespace.into(), class_name.into(), attrs)
                .expect("Could not add a typedef")
        }
    }
}
impl AsmBuilderHandle {
    pub fn new(asm_name: &str, main_module_name: &str, version: Version) -> Self {
        unsafe { new_assembly_def(asm_name.into(), main_module_name.into(), true, version) }
            .expect("Could not create a new assembly!")
    }
    pub fn add_typedef(self, def: TypeDefHandle) {
        unsafe { add_typedef(self.into(), def.into()) }
    }
    fn type_to_handle(&self, tpe: &Type) -> TypeHandle {
        match tpe {
            Type::I8 => unsafe { int8((*self).into()) },
            Type::I16 => unsafe { int16((*self).into()) },
            Type::I32 => unsafe { int32((*self).into()) },
            Type::U8 => unsafe { uint8((*self).into()) },
            Type::U16 => unsafe { uint16((*self).into()) },
            Type::U32 => unsafe { uint32((*self).into()) },
            Type::USize => unsafe { nuint((*self).into()) },
            Type::Bool => unsafe { get_bool((*self).into()) },
            Type::Void => unsafe { void((*self).into()) },
            Type::Ptr(handle) => unsafe { type_ref_to_pointer(self.type_to_handle(handle)) },
            Type::DotnetType(dotnet) => {
                if dotnet.asm().is_none() {
                    //TODO: change name path into namespace and name!
                    unsafe {
                        new_type_ref("".into(), dotnet.name_path().into(), dotnet.is_valuetype())
                    }
                } else {
                    todo!("Can't yet handle extern type refs")
                }
            }
            _ => todo!("Can't convert type {tpe:?} to System.Type yet!"),
        }
    }
    pub fn save(self, path: &str) {
        unsafe {
            serialize_assembly(self.into(), path.into());
        }
    }
}
#[test]
fn load_lib() {
    let builder = AsmBuilderHandle::new("MyAssembly", "CuteAsm", Version::default());
    builder.save("./test/Empty.dll");
}
#[test]
fn lib_simple_typedef() {
    let builder = AsmBuilderHandle::new("MyAssembly", "CuteAsm", Version::default());
    for idx in 0..1_000 {
        let simple = TypeDefHandle::new(
            "SpaceOfNames",
            &format!("KindOfTypeNo{idx}"),
            TypeAttributes::default(),
        );
        builder.add_typedef(simple);
    }
    builder.save("./test/SimpleTypedef.dll");
}
pub struct DotnetContext {
    asm: AsmBuilderHandle,
    main_class: TypeDefHandle,
    defs: Vec<TypeDefHandle>,
}
impl DotnetContext {
    fn into_typedef(&mut self, tpe: &crate::r#type::TypeDef) -> TypeDefHandle {
        let attrs = if tpe.extends().is_some() {
            TypeAttributes::default()
        } else {
            TypeAttributes::default().explicit()
        }
        .sealed()
        .public();
        let type_def = TypeDefHandle::new("", tpe.name(), attrs);
        for idx in 0..tpe.fields().len() {
            let field = &tpe.fields()[idx];
            let handle = self.asm.type_to_handle(&field.1);
            let field_def = unsafe {
                new_field_def(
                    field.0.as_ref().into(),
                    TypeAttributes::default(),
                    handle,
                    tpe.explicit_offsets().is_some(),
                    tpe.explicit_offsets()
                        .map(|offsets| offsets[idx])
                        .unwrap_or(0),
                )
            };
            unsafe { add_field_def(type_def, field_def) };
        }
        for inner in tpe.inner_types() {
            let inner = self.into_typedef(inner);
            unsafe { add_inner_type(type_def, inner) };
        }
        if let Some(_extends) = tpe.extends() {
            todo!("Type inheretence is not yet supported!")
        } else {
            unsafe { set_typedef_baseclass(type_def, valuetype(self.asm.into())) }
        }
        println!("Adding type def {tpe:?}");
        type_def
    }
}
impl AssemblyExporter for DotnetContext {
    fn init(asm_info: &super::AssemblyInfo) -> Self {
        let asm = AsmBuilderHandle::new(asm_info, "Rust", Version::default());
        let main_class = TypeDefHandle::new("", "<Module>", TypeAttributes::default());
        Self {
            asm,
            main_class,
            defs: Vec::with_capacity(0x100),
        }
    }

    fn add_type(&mut self, tpe: &crate::r#type::TypeDef) {
        self.asm.add_typedef(self.into_typedef(tpe));
    }

    fn add_method(&mut self, _method: &crate::method::Method) {
        //todo!("Method")
    }

    fn finalize(
        self,
        final_path: &std::path::Path,
        _is_dll: bool,
    ) -> Result<(), super::AssemblyExportError> {
        self.asm
            .save(final_path.as_os_str().to_str().expect("INVALID PATH"));
        //todo!()
        Ok(())
    }

    fn add_extern_ref(&mut self, _asm_name: &str, _info: &crate::assembly::AssemblyExternRef) {
        // Ignored for now!
        //todo!()
    }

    fn add_global(&mut self, _tpe: &crate::r#type::Type, _name: &str) {
        //todo!()
    }

    fn add_extern_method(&mut self, lib_path: &str, name: &str, sig: &crate::function_sig::FnSig) {
        todo!()
    }
}
