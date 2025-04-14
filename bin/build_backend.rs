use std::path::PathBuf;
#[macro_export]
macro_rules! config {
    ($name:ident,bool,$default:expr) => {
        pub static $name: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
            std::env::vars()
                .find_map(|(key, value)| {
                    if key == stringify!($name) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .map(|value| match value.as_ref() {
                    "0" | "false" | "False" | "FALSE" => false,
                    "1" | "true" | "True" | "TRUE" => true,
                    _ => panic!(
                        "Boolean enviroment variable {} has invalid value {}",
                        stringify!($name),
                        value
                    ),
                })
                .unwrap_or($default)
        });
    };
    ($name:ident,bool,$default:expr,$comment:literal) => {
        #[doc = $comment]
        pub static $name: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
            std::env::vars()
                .find_map(|(key, value)| {
                    if key == stringify!($name) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .map(|value| match value.as_ref() {
                    "0" | "false" | "False" | "FALSE" => false,
                    "1" | "true" | "True" | "TRUE" => true,
                    _ => panic!(
                        "Boolean enviroment variable {} has invalid value {}",
                        stringify!($name),
                        value
                    ),
                })
                .unwrap_or($default)
        });
    };
}
macro_rules! config_flag {
    ($var:ident,$default:expr) => {
        config! {$var,bool,$default}
    };
    ($var:ident,$default:expr,$comment:literal) => {
        config! {$var,bool,$default,$comment}
    };
}
#[must_use]
pub fn cargo_build_env() -> String {
    RUSTC_BUILD_STATUS.as_ref().expect("Could not build rustc!");
    let backend = absolute_backend_path();
    let backend = backend.display().to_string();
    let linker = RUSTC_CODEGEN_CLR_LINKER.display().to_string();
    let link_args = "--cargo-support";
    let radomize_layout = if *RANDOMIZE_LAYOUT {
        "-Z randomize-layout"
    } else {
        ""
    };
    format!("-Z codegen-backend={backend} -C linker={linker} -C link-args={link_args} {radomize_layout}")
}
pub fn ilasm_check() {
    match std::process::Command::new(&*ILASM_PATH).output(){
        Ok(_)=>println!("An CIL assembler has been detected."),
        Err(err)=>panic!("Could not find the CIL assembler at name/path {:?}, due to {err:?}.
Please instal the CIL assembler, and/or set the ILASM_PATH enviroment variable to point to your CIL assembler.",*ILASM_PATH)
    }
}
#[doc = "Specifies the path to the IL assembler."]
pub static ILASM_PATH: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::vars()
        .find_map(|(key, value)| {
            if key == "ILASM_PATH" {
                Some(value)
            } else {
                None
            }
        })
        .unwrap_or(get_default_ilasm())
});
/// Absolute path to the codegen backend shared library.
#[must_use]
pub fn absolute_backend_path() -> PathBuf {
    if cfg!(debug_assertions) {
        if cfg!(target_os = "linux") {
            std::fs::canonicalize("target/debug/librustc_codegen_clr.so").unwrap()
        } else if cfg!(target_os = "windows") {
            std::fs::canonicalize("target/debug/rustc_codegen_clr.dll").unwrap()
        } else if cfg!(target_os = "macos") {
            let current_dir = std::env::current_dir().unwrap();
            let dylib_path = current_dir.join("target/debug/librustc_codegen_clr.dylib");
            std::fs::canonicalize(dylib_path).expect("Could not find the backend dylib")
        } else {
            panic!("Unsupported target OS");
        }
    } else if cfg!(target_os = "linux") {
        std::fs::canonicalize("../target/release/librustc_codegen_clr.so").unwrap()
    } else if cfg!(target_os = "windows") {
        std::fs::canonicalize("../target/release/rustc_codegen_clr.dll").unwrap()
    } else if cfg!(target_os = "macos") {
        let current_dir = std::env::current_dir().unwrap();
        let dylib_path = current_dir.join("target/release/librustc_codegen_clr.dylib");
        std::fs::canonicalize(dylib_path).expect("Could not find the backend dylib")
    } else {
        panic!("Unsupported target OS");
    }
}

#[cfg(not(target_os = "windows"))]
/// Finds the default instance of the IL assembler.
fn get_default_ilasm() -> String {
    "ilasm".into()
}
#[cfg(target_os = "windows")]
fn get_default_ilasm() -> String {
    if std::process::Command::new("ilasm")
        .arg("--help")
        .output()
        .is_ok()
    {
        return "ilasm".into();
    }
    // Framework Path
    let framework_path = std::path::PathBuf::from("C:\\Windows\\Microsoft.NET\\Framework");
    let framework_dir = std::fs::read_dir(&framework_path).unwrap_or_else(|_| panic!("Could not find the .NET framework directory at {framework_path:?}, when searching for ilasm."));
    for entry in framework_dir {
        let entry = entry.unwrap();
        // TODO: find the most recent framework
        if entry.metadata().unwrap().is_dir() {
            let mut ilasm_path = entry.path();
            ilasm_path.push("ilasm");
            ilasm_path.set_extension("exe");
            if !std::fs::exists(&ilasm_path).unwrap_or(false) {
                eprintln!("Could not find ilasm at:{ilasm_path:?}");
                continue;
            }
            return ilasm_path.display().to_string();
        }
    }
    panic!("Could not find a .NET framework in directory {framework_path:?}, when searching for ilasm.")
}
static RUSTC_BUILD_STATUS: std::sync::LazyLock<Result<(), String>> =
    std::sync::LazyLock::new(build_backend);
static RUSTC_CODEGEN_CLR_LINKER: std::sync::LazyLock<PathBuf> = std::sync::LazyLock::new(|| {
    let _ = *RUSTC_BUILD_STATUS;
    if cfg!(debug_assertions) {
        std::process::Command::new("cargo")
            .args(["build", "--bin", "linker"])
            .output()
            .unwrap();
        //TODO: Fix this for other platforms
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let current_dir = std::env::current_dir().unwrap();
            let linker_path = current_dir.join("target/debug/linker");
            std::fs::canonicalize(linker_path).unwrap()
        } else if cfg!(target_os = "windows") {
            std::fs::canonicalize("../target/debug/linker.exe").unwrap()
        } else {
            panic!("Unsupported target OS");
        }
    } else {
        std::process::Command::new("cargo")
            .args(["build", "--bin", "linker", "--release"])
            .output()
            .unwrap();
        //TODO: Fix this for other platforms
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let current_dir = std::env::current_dir().unwrap();
            let linker_path = current_dir.join("target/release/linker");
            std::fs::canonicalize(linker_path).unwrap()
        } else if cfg!(target_os = "windows") {
            std::fs::canonicalize("../target/release/linker.exe").unwrap()
        } else {
            panic!("Unsupported target OS");
        }
    }
});
#[cfg(debug_assertions)]
fn build_backend() -> Result<(), String> {
    let _out = std::process::Command::new("cargo")
        .args(["build", "--lib"])
        .output()
        .map_err(|err| err.to_string())?;
    let _out = std::process::Command::new("cargo")
        .args(["build", "--lib", "--release"])
        .output()
        .map_err(|err| err.to_string())?;
    let current_dir = std::env::current_dir().unwrap();
    let _out = std::process::Command::new("cargo")
        .current_dir(current_dir.join("cilly"))
        .args(["build", "--bin", "linker"])
        .output()
        .expect("could not build the backend");
    let _out = std::process::Command::new("cargo")
        .current_dir(current_dir.join("cilly"))
        .args(["build", "--bin", "linker", "--release"])
        .output()
        .expect("could not build the backend");
    Ok(())
}
#[cfg(not(debug_assertions))]
fn build_backend() -> Result<(), String> {
    std::process::Command::new("cargo")
        .args(["build", "--release", "--lib"])
        .output()
        .expect("could not build the backend");
    std::process::Command::new("cargo")
        .args(["build", "--release", "--bin", "linker"])
        .output()
        .expect("could not build the backend");
    Ok(())
}

config_flag! {RANDOMIZE_LAYOUT,false,"Tells the codegen to randomize TEST type layout."}
