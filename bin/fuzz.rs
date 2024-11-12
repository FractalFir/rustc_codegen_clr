#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
rayon = "1.10.0"
strsim = "0.11.1"
---
use std::path::PathBuf;
#[must_use]
pub fn test_dotnet_executable(file_path: &str, test_dir: &str) -> String {
    use std::io::Write;
    if *DRY_RUN {
        return String::new();
    }
    #[cfg(not(target_os = "windows"))]
    assert!(
        (*IS_DOTNET_PRESENT || *IS_MONO_PRESENT),
        "You must have the dotnet runtime installed to run tests."
    );
    let exec_path = &format!("{file_path}.exe");
    #[cfg(target_os = "windows")]
    let exec_path = &std::fs::canonicalize(format!("{test_dir}//{exec_path}")).unwrap();
    let mut stdout = String::new();
    if *C_MODE {
        let out = std::process::Command::new("timeout")
            .current_dir(test_dir)
            .arg("-v")
            .arg("1")
            .arg(exec_path)
            .output()
            .expect("failed to run test program!");
        let stderr = String::from_utf8(out.stderr).expect("stderr is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Test program failed with message {stderr:}"
        );
        return String::from_utf8_lossy(&out.stdout).to_string();
    }

    //println!("exec_path:{exec_path:?}");
    if *IS_DOTNET_PRESENT {
        let config_path = if file_path.contains(test_dir) {
            format!("{file_path}.runtimeconfig.json")
        } else if cfg!(target_os = "windows") {
            format!("{test_dir}\\{file_path}.runtimeconfig.json")
        } else {
            format!("{test_dir}/{file_path}.runtimeconfig.json")
        };

        let mut file = std::fs::File::create(&config_path).unwrap_or_else(|err| {
            panic!("Could not create runtime config file at {config_path:?} due to {err:?}")
        });
        file.write_all(get_runtime_config().as_bytes())
            .expect("Could not write runtime config");
        //RUNTIME_CONFIG
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let mut cmd = {
            let mut cmd = std::process::Command::new("timeout");
            cmd.arg("-v");
            cmd.arg("5");
            cmd.arg("dotnet");
            cmd.arg(exec_path);
            cmd
        };
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let cmd = std::process::Command::new(exec_path);
            cmd
        };
        cmd.current_dir(test_dir);

    
        let out = cmd.output().expect("failed to run test assebmly!");

        let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Test program failed with message {stderr:}"
        );
        stdout = String::from_utf8_lossy(&out.stdout).to_string();
    }
    if *IS_MONO_PRESENT && *TEST_WITH_MONO {
        // Execute the test assembly
        let out = std::process::Command::new("mono")
            .current_dir(test_dir)
            .args([exec_path])
            .output()
            .expect("failed to run test assebmly!");
        let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Test program failed with message {stderr:}"
        );
    } else {
        #[cfg(not(target_os = "windows"))]
        assert!(*IS_DOTNET_PRESENT, "Only mono runtime present. Mono does not support all the features required to get Rust code working.");
    }

    stdout
}
fn backend_path() -> String {
    format!("codegen-backend={}", absolute_backend_path().display())
}
/// A list of arguments needed for invoking `rustc` with this backend included.
#[must_use]
pub fn rustc_args() -> Box<[String]> {
    if *RANDOMIZE_LAYOUT {
        [
            "-Z".to_owned(),
            backend_path(),
            "-C".to_owned(),
            format!("linker={}", RUSTC_CODEGEN_CLR_LINKER.display()),
            "-Z".to_owned(),
            "randomize-layout".to_owned(),
            "--edition".to_owned(),
            "2021".to_owned(),
        ]
        .into()
    } else {
        [
            "-Z".to_owned(),
            backend_path(),
            "-C".to_owned(),
            format!("linker={}", RUSTC_CODEGEN_CLR_LINKER.display()),
            "--edition".to_owned(),
            "2021".to_owned(),
        ]
        .into()
    }
}
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
            std::fs::canonicalize("../target/debug/librustc_codegen_clr.so").unwrap()
        } else if cfg!(target_os = "windows") {
            std::fs::canonicalize("../target/debug/rustc_codegen_clr.dll").unwrap()
        } else if cfg!(target_os = "macos") {
            std::fs::canonicalize("../target/debug/librustc_codegen_clr.dylib").unwrap()
        } else {
            panic!("Unsupported target OS");
        }
    } else if cfg!(target_os = "linux") {
        std::fs::canonicalize("../target/release/librustc_codegen_clr.so").unwrap()
    } else if cfg!(target_os = "windows") {
        std::fs::canonicalize("../target/release/rustc_codegen_clr.dll").unwrap()
    } else if cfg!(target_os = "macos") {
        std::fs::canonicalize("../target/release/librustc_codegen_clr.dylib").unwrap()
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
            std::fs::canonicalize("../target/debug/linker").unwrap()
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
            std::fs::canonicalize("../target/release/linker").unwrap()
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
    let _out = std::process::Command::new("cargo")
        .current_dir("../cilly")
        .args(["build", "--bin", "linker"])
        .output()
        .expect("could not build the backend");
    let _out = std::process::Command::new("cargo")
        .current_dir("../cilly")
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

use std::{io::Write, process::Command, sync::atomic::AtomicU64};
static LINES: AtomicU64 = AtomicU64::new(0);
fn run_test(test_id: u64, is_release: bool) -> Option<f64> {
    match std::panic::catch_unwind(|| run_test_impl(test_id, is_release)) {
        Ok(inner) => inner,
        Err(_) => Some(1.0),
    }
}
fn run_test_impl(test_id: u64, is_release: bool) -> Option<f64> {
    let rustc_opt_flag = if is_release { "-O" } else { "-g" };
    let test_dir = "/tmp/fuzz/";
    let rust_src = format!("/tmp/fuzz/fuzz{test_id}.rs");
    let dotnet_exe = format!("/tmp/fuzz/fuzz{test_id}.exe");
    let dotnet_wrapper = format!("/tmp/fuzz/fuzz{test_id}");
    let native_exec = format!("/tmp/fuzz/fuzz{test_id}.elf");
    // Ensures the test directory is present
    std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
    // Builds the backend if neceasry
    RUSTC_BUILD_STATUS
        .as_ref()
        .expect("Could not build rustc!");
    // Compiles the test project
    let mut cmd = std::process::Command::new("rustc");
    //.env("RUST_TARGET_PATH","../../")
    let rustc_args = rustc_args();
    cmd.current_dir(test_dir)
        .arg("-O")
        .args(rustc_args.iter())
        .args([&rust_src, "-o", &dotnet_exe]);

    let out = cmd.output().expect("failed to execute process");
    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");
        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
    }

    //super::peverify(exec_path, test_dir);

    let dotnet_out =
        test_dotnet_executable(&dotnet_wrapper, test_dir);
    // Compiles the project with native rust
    let mut cmd = std::process::Command::new("rustc");
    //.env("RUST_TARGET_PATH","../../")
    cmd.current_dir(test_dir).args([
        rustc_opt_flag,
        &rust_src,
        "-o",
        &native_exec,
        "--edition",
        "2021",
    ]);
    let out = cmd.output().expect("failed to execute process");
    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");

        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
    }
    let rust_out = std::process::Command::new(&native_exec)
        .current_dir(test_dir)
        .output()
        .expect("failed to execute process");
    let rust_out =
        String::from_utf8(rust_out.stdout).expect("rust error contained non-UTF8 characters.");

    if rust_out == dotnet_out {
        //std::fs::remove_file(rust_src).unwrap();
        std::fs::remove_file(dotnet_exe).unwrap();
        std::fs::remove_file(native_exec).unwrap();
        //std::fs::remove_file(dotnet_wrapper).unwrap();
        None
    } else {
        eprintln!("rust_out:\n\n{rust_out}dotnet_out:\n\n{dotnet_out}");
        Some(strsim::jaro(&rust_out, &dotnet_out))
    }
}
fn gen_file(test_id: u64, generator: &str) {
    let rust_src = format!("/tmp/fuzz/fuzz{test_id}.rs");
    let cout = Command::new(generator)
        .arg("-p")
        .arg(&format!("{test_id}"))
        .output()
        .unwrap();
    assert!(cout.stderr.is_empty());
    let src = cout.stdout;
    LINES.fetch_add(
        src.iter().filter(|byte| **byte == b'\n').count() as u64,
        std::sync::atomic::Ordering::Relaxed,
    );
    let mut file = std::fs::File::create(rust_src).unwrap();
    file.write_all(b"#![allow(dead_code,unused_variables)]")
        .unwrap();
    file.write_all(&src).unwrap();
}
fn test(test_id: u64, generator: &str) -> Option<(u64, f64)> {
    gen_file(test_id, generator);
    let res = run_test(test_id, false)
        .or(run_test(test_id, true))
        .map(|sim| (test_id, 1.0 - sim));
    if res.is_none() {
        std::fs::remove_file(format!("/tmp/fuzz/fuzz{test_id}.rs")).unwrap();
    }
    std::fs::remove_file(format!("/tmp/fuzz/fuzz{test_id}.il")).unwrap();
    std::fs::remove_file(format!("/tmp/fuzz/fuzz{test_id}.runtimeconfig.json")).unwrap();
    // Try removing the .mdb, if present.
    let _ = std::fs::remove_file(format!("/tmp/fuzz/fuzz{test_id}.exe.mdb"));
    res
}
fn main() {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    let generator = std::env::args().nth(1).unwrap();
    let search_start = str::parse::<u64>(&std::env::args().nth(2).unwrap()).unwrap();
    let search_end = std::env::args()
        .nth(3)
        .as_ref()
        .map_or(search_start + 1, |str| str::parse::<u64>(str).unwrap());

    std::fs::create_dir_all("/tmp/fuzz").unwrap();
    let mut faliures: Box<[_]> = (search_start..search_end)
        .into_par_iter()
        .map(|i| test(i, &generator))
        .flatten()
        .collect();
    faliures.sort_by(|(_, err_a), (_, err_b)| err_a.partial_cmp(err_b).unwrap());
    let test_cases = search_end - search_start;
    println!(
        "Created {test_cases} test cases, totaling {LINES:?} LOC, found faliures:{faliures:?}"
    );
}
config_flag! {DRY_RUN,false,"Tells the codegen test suite to not execute or link any test code, enabling testing on platforms without the .NET runtime present."}
config_flag! {C_MODE,false,"Tells the codegen to emmit C source files."}
config_flag! {TEST_WITH_MONO,false,"Tells the codegen to use the mono runtime for tests."}
#[cfg(target_os = "windows")]
const IS_DOTNET_PRESENT: &bool = &true;

#[cfg(not(target_os = "windows"))]
static IS_DOTNET_PRESENT: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::process::Command::new("dotnet").output().is_ok());
static IS_MONO_PRESENT: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::process::Command::new("mono").output().is_ok());
    /// Cached runtime configuration string, obtained from calling the .NET runtime.
#[must_use]
pub fn get_runtime_config() -> &'static str {
    RUNTIME_CONFIG.as_ref()
}

/// Cached runtime configuration file, obtained from calling the .NET runtime.
static RUNTIME_CONFIG: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
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
          \"tfm\": \"net8.0\",
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
});
