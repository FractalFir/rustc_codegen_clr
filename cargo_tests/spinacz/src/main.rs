#![feature(unsized_const_params)]
use mycorrhiza::intrinsics::RustcCLRInteropManagedArray;
use mycorrhiza::intrinsics::RustcCLRInteropManagedClass;
use mycorrhiza::system::MString;

use mycorrhiza::{
    System::Reflection::Assembly, System::Reflection::AssemblyName, System::Reflection::MemberInfo,
    System::Reflection::MethodInfo, System::Type,
};
use std::collections::HashMap;
use std::io::Write;

fn main() {
    let asm_name = std::env::args().nth(1).unwrap();
    let mut root_asm = Namespace::new(asm_name.replace(".", "_"), 0);
    let asm_name: MString = <&str as Into<MString>>::into(&asm_name);
    let asm = Assembly::static1::<"Load", MString, Assembly>(asm_name);
    let types = Assembly::virt0::<"GetTypes", RustcCLRInteropManagedArray<Type, 1>>(asm);
    let types_len = types.len();
    let mut idx = 0;
    let mut out = std::fs::File::create("out.rs").unwrap();

    while idx < types_len {
        let tpe = types.index(idx);
        let tpe_asm = type_asm_string(tpe);
        let inherits = Type::virt0::<"get_BaseType", Type>(tpe);
        //if inherits.equality()
        let inherits: String = if inherits.is_null() {
            "".into()
        } else {
            mstring_to_string(Type::virt0::<"get_FullName", MString>(inherits))
        };
        let name = mstring_to_string(Type::virt0::<"get_FullName", MString>(tpe));
        eprintln!("name:{name}");
        let is_valuetype = (Type::virt0::<"get_IsValueType", bool>(tpe));
        if name.contains("`") || name.contains("+") || name.contains("<") {
            idx += 1;
            continue;
        }
        root_asm.add_tpe(
            DotNetTypeDef {
                asm: tpe_asm,
                full_name: name.clone(),
                is_valuetype,
                inherits,
            },
            &name,
        );

        idx += 1;
    }
    println!("Prepraing for export");
    root_asm.export(&mut out);
    println!("Loaded the assembly, {types_len} types found! ");
}
fn type_asm_string(tpe: Type) -> String {
    mstring_to_string(AssemblyName::virt0::<"get_Name", MString>(
        Assembly::virt0::<"GetName", AssemblyName>(Type::virt0::<"get_Assembly", Assembly>(tpe)),
    ))
}
fn mstring_to_string(mstr: MString) -> String {
    use mycorrhiza::system::runtime::interop_services::Marshal;

    let ptr = Marshal::static1::<"StringToCoTaskMemUTF8", MString, isize>(mstr);
    if ptr == 0 {
        return "".into();
    }
    let s = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) }
        .to_str()
        .unwrap()
        .to_owned();
    Marshal::static1::<"FreeCoTaskMem", isize, ()>(ptr);
}
//pub fn method_sig()
enum DType {
    Ptr(Box<Self>),
    Class(String),
}
impl DType {
    pub fn from_tpe(tpe: Type) -> Self {
        let name = mstring_to_string(Type::virt0::<"get_FullName", MString>(tpe));
        Self::from_str(name)
    }
    fn from_str(name: &str) -> Self {
        Self::Class(name.into())
    }
}
type Sig = (Vec<DType>, DType);
enum MethodKind {
    Static,
    Instance,
    Virtual,
}
struct DotNetTypeDef {
    full_name: String,
    asm: String,
    is_valuetype: bool,
    inherits: String, //methods: HashMap<String, Vec<(Sig, MethodKind)>>,
}
struct Namespace {
    inner: HashMap<String, Self>,
    types: Vec<DotNetTypeDef>,
    name: String,
    depth: u32,
}
impl Namespace {
    pub fn add_tpe(&mut self, tpe: DotNetTypeDef, full_name_: &str) {
        let mut full_name = full_name_.split(".");
        let curr = full_name.next().unwrap();
        if let Some(next) = full_name.next() {
            self.inner
                .entry(curr.to_string())
                .or_insert_with(|| Namespace::new(curr.to_string(), self.depth + 1))
                .add_tpe(tpe, full_name_.split_once('.').unwrap().1)
        } else {
            self.types.push(tpe);
        }
    }
    pub fn new(name: String, depth: u32) -> Self {
        Self {
            name,
            types: vec![],
            inner: HashMap::default(),
            depth,
        }
    }
    pub fn export(&self, out: &mut impl Write) {
        writeln!(out, "pub mod {name}{{", name = self.name).unwrap();
        for (_, inner) in &self.inner {
            inner.export(out);
        }
        for tpe in &self.types {
            if !tpe.is_valuetype {
                let name = tpe.full_name.split('.').last().unwrap();
                writeln!(out,"pub type {name} =  mycorrhiza::intrinsics::RustcCLRInteropManagedClass<{tpe_asm:?},{full_name:?}>;",tpe_asm = tpe.asm,full_name = tpe.full_name ).unwrap();
                if self.depth > 0 {
                    writeln!(
                        out,
                        "use {}*;",
                        (0..self.depth)
                            .into_iter()
                            .map(|_| "super::")
                            .collect::<String>()
                    );
                }

                if !tpe.inherits.is_empty()
                    && !(tpe.inherits.contains("`")
                        || tpe.inherits.contains("+")
                        || tpe.inherits.contains("<"))
                {
                    writeln!(
                        out,
                        "impl From<{name}> for {inherits_path} {{\n fn from(v:{name})->{inherits_path}{{\nmycorrhiza::intrinsics::rustc_clr_interop_managed_checked_cast::<{inherits_path},{name}>(v)\n}}}} ",
                        inherits_path = tpe.inherits.replace(".", "::")
                    )
                    .unwrap();
                }
            }
        }
        writeln!(out, "}}").unwrap();
    }
}
