#![feature(unsized_const_params)]
use mycorrhiza::intrinsics::RustcCLRInteropManagedArray;
use mycorrhiza::intrinsics::RustcCLRInteropManagedClass;
use mycorrhiza::system::MString;
use std::collections::HashMap;
use std::io::Write;
pub type Assembly = mycorrhiza::intrinsics::RustcCLRInteropManagedClass<
    "System.Reflection",
    "System.Reflection.Assembly",
>;
pub type AssemblyName = mycorrhiza::intrinsics::RustcCLRInteropManagedClass<
    "System.Reflection",
    "System.Reflection.AssemblyName",
>;
pub type Type =
    mycorrhiza::intrinsics::RustcCLRInteropManagedClass<"System.Runtime", "System.Type">;
pub type MemberInfo = mycorrhiza::intrinsics::RustcCLRInteropManagedClass<
    "System.Runtime",
    "System.Reflection.MemberInfo",
>;
fn main() {
    let asm_name = std::env::args().nth(1).unwrap();
    let mut root_asm = Namespace::new(asm_name.replace(".", "_"));
    let asm_name: MString = <&str as Into<MString>>::into(&asm_name);
    let asm = Assembly::static1::<"Load", MString, Assembly>(asm_name);
    let types = Assembly::virt0::<"GetTypes", RustcCLRInteropManagedArray<Type, 1>>(asm);
    let types_len = types.len();
    let mut idx = 0;
    let mut out = std::fs::File::create("out.rs").unwrap();

    while idx < types_len {
        let tpe = types.index(idx);
        let tpe_asm = mstring_to_string(AssemblyName::virt0::<"get_Name", MString>(
            Assembly::virt0::<"GetName", AssemblyName>(Type::virt0::<"get_Assembly", Assembly>(
                tpe,
            )),
        ));

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
            },
            &name,
        );

        idx += 1;
    }
    root_asm.export(&mut out);
    println!("Loaded the assembly, {types_len} types found!");
}
fn mstring_to_string(mstr: MString) -> String {
    use mycorrhiza::system::runtime::interop_services::Marshal;

    let ptr = Marshal::static1::<"StringToCoTaskMemUTF8", MString, isize>(mstr);
    let s = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) }
        .to_str()
        .unwrap()
        .to_owned();
    Marshal::static1::<"FreeCoTaskMem", isize, ()>(ptr);
    s
}

struct DotNetTypeDef {
    full_name: String,
    asm: String,
    is_valuetype: bool,
}
struct Namespace {
    inner: HashMap<String, Self>,
    types: Vec<DotNetTypeDef>,
    name: String,
}
impl Namespace {
    pub fn add_tpe(&mut self, tpe: DotNetTypeDef, full_name_: &str) {
        let mut full_name = full_name_.split(".");
        let curr = full_name.next().unwrap();
        if let Some(next) = full_name.next() {
            self.inner
                .entry(curr.to_string())
                .or_insert_with(|| Namespace::new(curr.to_string()))
                .add_tpe(tpe, full_name_.split_once('.').unwrap().1)
        } else {
            self.types.push(tpe);
        }
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            types: vec![],
            inner: HashMap::default(),
        }
    }
    pub fn export(&self, out: &mut impl Write) {
        writeln!(out, "mod {name}{{", name = self.name).unwrap();
        for (_, inner) in &self.inner {
            inner.export(out);
        }
        for tpe in &self.types {
            if !tpe.is_valuetype {
                writeln!(out,"pub type {name} =  mycorrhiza::intrinsics::RustcCLRInteropManagedClass<{tpe_asm:?},{full_name:?}>;",name = tpe.full_name.split('.').last().unwrap(),tpe_asm = tpe.asm,full_name = tpe.full_name ).unwrap();
            }
        }
        writeln!(out, "}}").unwrap();
    }
}
