use crate::{
    access_modifier::AccessModifer,
    asm::{Assembly, AssemblyExternRef},
    asm_exporter::{AssemblyExportError, AssemblyExporter},
    ilasm_op::{non_void_type_cil, type_cil},
    method::Method,
    type_def::TypeDef,
    IString, IlasmFlavour, Type,
};
use lazy_static::*;
use std::fmt::Write;

#[must_use]
/// A struct used to export an asssembly using the ILASM tool as a .NET assembly creator.
pub struct ILASMExporter {
    encoded_asm: String,
    methods: String,
    init_locals: bool,
    flavour: IlasmFlavour,
    ilasm_path: IString,
    escape_names: bool,
    runtime_config: IString,
    print_stack_traces: bool,
}
impl Default for ILASMExporter {
    fn default() -> Self {
        Self::init(
            "rust_mod",
            always_init_locals(),
            *ILASM_FLAVOUR,
            *PRINT_PTRS,
            ilasm_path(),
            *ESCAPE_NAMES,
            get_runtime_config(),
            *TRACE_CALLS,
        )
    }
}
/// Cached runtime configuration string, obtained from calling the .NET runtime.
#[must_use]
pub fn get_runtime_config() -> &'static str {
    RUNTIME_CONFIG.as_ref()
}
lazy_static! {
  /// Cached runtime configuration file, obtained from calling the .NET runtime.
  static ref RUNTIME_CONFIG: String = {
    let info = std::process::Command::new("dotnet")
        .arg("--info")
        .output()
        .expect("Could not run `dotnet --info`");
    if !info.stderr.is_empty() {
        let stderr = std::str::from_utf8(&info.stderr).expect("Error message not utf8");
        panic!("dotnet --info panicked with {stderr}")
    }
    let info = std::str::from_utf8(&info.stdout).expect("Error message not utf8");
    let version_start = info.find("Host:").unwrap_or_default();
    let version_start = version_start + info[version_start..].find("Version:").unwrap();
    let version_start = version_start + "Version:".len();
    let version_end = info.find("Architecture:").unwrap();
    let version = &info[version_start..version_end].trim();
    format!(
        "{{
        \"runtimeOptions\": {{
          \"tfm\": \"netcoreapp3.1\",
          \"framework\": {{
            \"name\": \"Microsoft.NETCore.App\",
            \"version\": \"{version}\"
          }},
          \"configProperties\": {{
            \"System.Threading.ThreadPool.MinThreads\": 4,
            \"System.Threading.ThreadPool.MaxThreads\": 25
          }}
        }}
      }}"
    )
    };
}
lazy_static! {
    #[doc = "Tells the codegen to escape class and method names."]pub static ref ESCAPE_NAMES:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(ESCAPE_NAMES){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(ESCAPE_NAMES),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    pub static ref ILASM_FLAVOUR: IlasmFlavour = {
        if String::from_utf8_lossy(
            &std::process::Command::new(&*ILASM_PATH)
                .output()
                .unwrap()
                .stdout,
        )
        .contains("PDB")
        {
            IlasmFlavour::Modern
        } else {
            IlasmFlavour::Clasic
        }
    };
}

lazy_static! {
    #[doc = "Specifies the path to the IL assembler."]
    pub static ref ILASM_PATH:String = {
        std::env::vars().find_map(|(key,value)|if key == "ILASM_PATH"{Some(value)}else{None}).unwrap_or("ilasm".into())
    };
}

lazy_static! {
    #[doc = "Changes `.locals` into `.locals init`. Causes the runtime to always initialize local variables.\nTry turining on in cause of issues. If it fixes them, then their root cause is use of uninitailized memory."]pub static ref ALWAYS_INIT_LOCALS:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(ALWAYS_INIT_LOCALS){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(ALWAYS_INIT_LOCALS),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Tells codegen the print each pointer it dereferences."]
    pub static ref PRINT_PTRS:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(PRINT_PTRS){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(PRINT_PTRS),value),
        }).unwrap_or(false)
    };
}

pub fn ilasm_path() -> &'static str {
    ILASM_PATH.as_str()
}
pub fn always_init_locals() -> bool {
    *ALWAYS_INIT_LOCALS
}
impl ILASMExporter {
    pub fn init(
        asm_name: &str,
        init_locals: bool,
        flavour: IlasmFlavour,
        print_ptrs: bool,
        ilasm_path: &str,
        escape_names: bool,
        runtime_config: &str,
        print_stack_traces: bool,
    ) -> Self {
        let mut encoded_asm = String::with_capacity(0x1_00);
        let mut methods = String::with_capacity(0x1_00);
        write!(encoded_asm, ".assembly {asm_name}{{}}").expect("Write error!");
        writeln!(methods, ".class beforefieldinit RustModule{{").expect("Write error!");
        if print_ptrs {
            write!(methods, ".method public static native uint watch_ptr(native uint){{ldstr \"Derefing ptr:\"\ncall void [System.Console]System.Console::Write(string)\nldarg.0\nconv.u8\ncall void [System.Console]System.Console::WriteLine(uint64)\nldarg.0\nret\n}}\n").expect("Write error!");
        }
        write!(methods, ".method public static native uint check_calli_nonull(native uint){{\nldarg.0\nbrnull FAILURE\nldarg.0\nret\nFAILURE:newobj instance void [System.Runtime]System.NullReferenceException::.ctor()\nthrow}}\n").expect("Write error!");
        Self {
            encoded_asm,
            methods,
            init_locals,
            flavour,
            ilasm_path: ilasm_path.into(),
            escape_names,
            runtime_config: runtime_config.into(),
            print_stack_traces,
        }
    }
}

impl AssemblyExporter for ILASMExporter {
    fn add_global(&mut self, tpe: &Type, name: &str, thread_local: bool, info: &Assembly) {
        if thread_local{
            writeln!(
                self.methods,
                ".field static {tpe} '{name}'\n.custom instance void [System.Runtime]System.ThreadStaticAttribute::.ctor() = (01 00 00 00)",
                tpe = super::ilasm_op::non_void_type_cil(tpe,info.string_map())
            )
        }
       else{
        writeln!(
            self.methods,
            ".field static {tpe} '{name}'",
            tpe = super::ilasm_op::non_void_type_cil(tpe,info.string_map())
        )
       }
        .expect("Could not write global!");
    }

    fn add_extern_ref(&mut self, asm_name: &str, asm_ref_data: &AssemblyExternRef) {
        let (v1, v2, v3, v4) = asm_ref_data.version();
        write!(
            self.encoded_asm,
            ".assembly extern {asm_name}{{.ver {v1}:{v2}:{v3}:{v4} }}"
        )
        .expect("Write error!");
    }
    fn add_type(&mut self, tpe: &TypeDef, asm: &Assembly) {
        type_def_cli(
            &mut self.encoded_asm,
            tpe,
            false,
            self.escape_names,
            self.flavour,
            self.init_locals,
            self.print_stack_traces,
            asm,
        )
        .expect("Error");
        //let _ = self.types.push(tpe.clone());
    }
    fn add_method(&mut self, method: &Method, asm: &Assembly) {
        method
            .export(
                &mut self.methods,
                self.flavour,
                self.init_locals,
                self.print_stack_traces,
                asm,
            )
            .expect("Error");
    }
    fn finalize(
        self,
        final_path: &std::path::Path,
        is_dll: bool,
    ) -> Result<(), AssemblyExportError> {
        use std::io::Write;
        let mut out_path = final_path.to_owned();
        //out_path.set_file_name(final_path.file_name().expect("Target file has no name!"));
        if let Some(ext) = final_path.extension() {
            out_path = out_path.with_extension(ext);
        }
        //final_path.expect("Could not canonialize path!");

        let cil_path = final_path.with_extension("il");
        let config = final_path.with_extension("runtimeconfig.json");
        let mut config = std::fs::File::create(config).unwrap();
        config
            .write_all(self.runtime_config.as_bytes())
            .expect("Could not write runtime config");

        let mut cil = self.encoded_asm;
        cil.push_str(&self.methods);
        writeln!(cil, "}}")?;
        match std::fs::File::create(&cil_path) {
            Ok(ok) => ok,
            Err(err) => panic!("Can't create file at path {cil_path:?} because {err:?}"),
        }
        .write_all(cil.as_bytes())
        .expect("Could not write bytes");
        let asm_type = if is_dll { "-dll" } else { "-exe" };
        let target = format!(
            "-output:{out_path}",
            out_path = out_path.clone().to_string_lossy()
        );
        let args: [String; 6] = [
            asm_type.into(),
            target,
            cil_path.clone().to_string_lossy().to_string(),
            "-debug".into(),
            "-OPTIMIZE".into(),
            "-FOLD".into(),
        ];

        let out = std::process::Command::new(AsRef::<str>::as_ref(&self.ilasm_path))
            .args(args)
            .output()
            .expect("failed run ilasm process");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("\nError\n") {
            let err = format!(
                "stdout:{} stderr:{}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
            );
            return Err(AssemblyExportError::ExporterError(err.into()));
        }
        Ok(())
    }

    fn add_extern_method(
        &mut self,
        lib_path: &str,
        name: &str,
        sig: &crate::fn_sig::FnSig,
        preserve_errno: bool,
        info: &Assembly,
    ) {
        // /lib64/libc.so.6
        let output = type_cil(sig.output(), info.string_map());
        let preserve_errno = if preserve_errno { "lasterr" } else { "" };
        writeln!(
            self.methods,
            ".method public hidebysig static pinvokeimpl(\"{lib_path}\" cdecl {preserve_errno}) {output} '{name}' ("
        )
        .unwrap();
        let mut input_iter = sig.inputs().iter();
        if let Some(input) = input_iter.next() {
            write!(
                self.methods,
                "{}",
                non_void_type_cil(input, info.string_map())
            )
            .unwrap();
        }
        for input in input_iter {
            write!(
                self.methods,
                ",{}",
                non_void_type_cil(input, info.string_map())
            )
            .unwrap();
        }
        // The `preservesig` is STRICTLY necesary - without it, the runtime sometimes replaces the result value with a HRESULT.
        writeln!(self.methods, ") preservesig {{}}").unwrap();
    }
}
fn type_def_cli(
    w: &mut impl Write,
    tpe: &TypeDef,
    is_nested: bool,
    escape_names: bool,
    flavour: IlasmFlavour,
    init_locals: bool,
    print_stack_traces: bool,
    asm: &Assembly,
) -> std::fmt::Result {
    let name = tpe.name();
    let name = if escape_names {
        crate::asm_exporter::escape_class_name(name)
    } else {
        name.into()
    };
    assert!(
        tpe.gargc() == 0,
        "Generic typedefs not supported yet. tpe:{tpe:?}"
    );
    let extends: IString = if let Some(extended) = tpe.extends() {
        crate::ilasm_op::dotnet_type_ref_extends(extended, asm.string_map()).into()
    } else {
        "[System.Runtime]System.ValueType".into()
    };
    let access = if let AccessModifer::Public = tpe.access_modifier() {
        "public"
    } else {
        "private"
    };
    let sealed = if tpe.explicit_offsets().is_some() || tpe.extends().is_none() {
        "sealed"
    } else {
        ""
    };
    let explicit = if tpe.explicit_offsets().is_some() || tpe.explict_size().is_some() {
        "explicit"
    } else {
        ""
    };
    let nested = if is_nested { "nested" } else { "" };
    writeln!(
        w,
        ".class {nested} {access} {explicit} ansi {sealed} '{name}' extends {extends}{{"
    )?;
    if let Some(size) = tpe.explict_size() {
        writeln!(w, ".size {size}")?;
    }
    for inner_type in tpe.inner_types() {
        type_def_cli(
            w,
            inner_type,
            true,
            escape_names,
            flavour,
            init_locals,
            print_stack_traces,
            asm,
        )?;
    }
    if let Some(offsets) = tpe.explicit_offsets() {
        for ((field_name, field_type), offset) in tpe.fields().iter().zip(offsets.iter()) {
            writeln!(
                w,
                "\t.field [{offset}] public {field_type_name} '{field_name}'",
                field_type_name = non_void_type_cil(field_type, asm.string_map())
            )?;
        }
    } else {
        for (field_name, field_type) in tpe.fields() {
            writeln!(
                w,
                "\t.field public {field_type_name} '{field_name}'",
                field_type_name = non_void_type_cil(field_type, asm.string_map())
            )?;
        }
    }
    for method in tpe.methods() {
        method.export(w, flavour, init_locals, print_stack_traces, asm)?;
    }
    writeln!(w, "}}")?;
    Ok(())
}

lazy_static! {
    #[doc = "Preapends each function call with a debug message"]pub static ref TRACE_CALLS:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(TRACE_CALLS){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(TRACE_CALLS),value),
        }).unwrap_or(false)
    };
}
