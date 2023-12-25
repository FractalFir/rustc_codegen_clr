use std::path::PathBuf;

#[cfg(test)]
fn peverify(file_path: &str, test_dir: &str) {
    if !*IS_PEVERIFY_PRESENT {
        //No PEVerify, can't check assemblies.
        let out = std::process::Command::new("peverify")
            .current_dir(test_dir)
            .args([file_path])
            .output()
            .expect("failed to verfiy test assebmly!");
        let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Verification of a test program failed with message {stderr:}"
        );
    }
}
#[cfg(test)]
fn test_dotnet_executable(file_path: &str, test_dir: &str) {
    use std::io::Write;

    let exec_path = &format!("{file_path}.exe");
    if *IS_MONO_PRESENT {
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
    }
    //println!("exec_path:{exec_path:?}");
    if *IS_DOTNET_PRESENT {
        let config_path = format!("{test_dir}/{file_path}.runtimeconfig.json");
        println!("{config_path:?}");
        let mut file = std::fs::File::create(config_path).unwrap();
        file.write_all(RUNTIME_CONFIG.as_bytes())
            .expect("COuld not write runtime config");
        //RUNTIME_CONFIG
        let out = std::process::Command::new("dotnet")
            .current_dir(test_dir)
            .args([exec_path])
            .output()
            .expect("failed to run test assebmly!");

        let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Test program failed with message {stderr:}"
        );
    }
    assert!(
        (*IS_DOTNET_PRESENT || *IS_MONO_PRESENT),
        "You must have either mono or dotnet runtime installed to run tests."
    );
}
macro_rules! test_lib {
    ($test_name:ident) => {
        mod $test_name {
            #[cfg(test)]
            static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            #[test]
            fn release() {
                // Ensures no two compilations run at the same time.
                let lock = COMPILE_LOCK.lock();
                // Ensures the test directory is present
                std::fs::create_dir_all("./test/out").expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                // Compiles the test project
                let mut command = std::process::Command::new("rustc");
                let command = command
                    .current_dir("./test/out")
                    //.env("RUST_TARGET_PATH","../../")
                    .args([
                        "-O",
                        "--crate-type=lib",
                        "-Z",
                        super::backend_path(),
                        "-C",
                        &format!("linker={}", super::RUSTC_CODEGEN_CLR_LINKER.display()),
                        concat!("../", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".rlib"),
                        //"--target",
                        // "clr64-unknown-clr"
                    ]);

                let command = if *super::IS_MONO_PRESENT {
                    // Tell the linker to test AOT
                    command.args(["-C", "link-arg=--aot-mode,mono-full"])
                } else {
                    command
                };
                let out = command.output().expect("failed to execute process");
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                let test_dll = concat!("./", stringify!($test_name), ".dll");
                let out = std::process::Command::new(
                    super::RUSTC_CODEGEN_CLR_LINKER.display().to_string(),
                )
                .current_dir("./test/out")
                .arg("-o")
                .arg(test_dll)
                .arg(concat!("./", stringify!($test_name), ".rlib"))
                .output()
                .unwrap();
                super::peverify(test_dll, "./test/out");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                drop(lock);
            }
            #[test]
            fn debug() {
                let lock = COMPILE_LOCK.lock();
                // Ensures the test directory is present
                std::fs::create_dir_all("./test/out").expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                // Compiles the test project
                let mut command = std::process::Command::new("rustc");
                let command = command
                    .current_dir("./test/out")
                    //.env("RUST_TARGET_PATH","../../")
                    .args([
                        "--crate-type=lib",
                        "-Z",
                        super::backend_path(),
                        "-C",
                        &format!("linker={}", super::RUSTC_CODEGEN_CLR_LINKER.display()),
                        concat!("../", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".rlib"),
                        //"--target",
                        // "clr64-unknown-clr"
                    ]);

                let command = if *super::IS_MONO_PRESENT {
                    // Tell the linker to test AOT
                    command.args(["-C", "link-arg=--aot-mode,mono-full"])
                } else {
                    command
                };
                let out = command.output().expect("failed to execute process");
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                let test_dll = concat!("./dbg_", stringify!($test_name), ".dll");
                let out = std::process::Command::new(
                    super::RUSTC_CODEGEN_CLR_LINKER.display().to_string(),
                )
                .current_dir("./test/out")
                .arg("-o")
                .arg(test_dll)
                .arg(concat!("./", stringify!($test_name), ".rlib"))
                .output()
                .unwrap();
                super::peverify(test_dll, "./test/out");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                drop(lock);
            }
        }
    };
}
macro_rules! run_test {
    ($prefix:ident,$test_name:ident) => {
        mod $test_name {
            #[cfg(test)]
            use ntest::timeout;
            #[cfg(test)]
            static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            #[test]
            #[timeout(30_000)]
            fn release() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./test/", stringify!($prefix), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                // Compiles the test project
                let out = std::process::Command::new("rustc")
                    //.env("RUST_TARGET_PATH","../../")
                    .current_dir(test_dir)
                    .args([
                        "-O",
                        "-Z",
                        super::backend_path(),
                        "-C",
                        &format!("linker={}", super::RUSTC_CODEGEN_CLR_LINKER.display()),
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".exe"),
                        //"--target",
                        //"clr64-unknown-clr"
                    ])
                    .output()
                    .expect("failed to execute process");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                let exec_path = concat!("../", stringify!($test_name));
                drop(lock);
                super::peverify(exec_path, test_dir);
                super::test_dotnet_executable(exec_path, test_dir);
            }
            #[test]
            #[timeout(30_000)]
            fn debug() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./test/", stringify!($prefix), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                let test_name = concat!("debug_", stringify!($test_name));
                let output_path = format!("./{test_name}.exe");
                // Compiles the test project
                let out = std::process::Command::new("rustc")
                    //.env("RUST_TARGET_PATH","../../")
                    .current_dir(test_dir)
                    .args([
                        "-Z",
                        super::backend_path(),
                        "-C",
                        &format!("linker={}", super::RUSTC_CODEGEN_CLR_LINKER.display()),
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        &output_path,
                        //"--target",
                        //"clr64-unknown-clr"
                    ])
                    .output()
                    .expect("failed to execute process");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                }
                let exec_path = format!("../{test_name}");
                drop(lock);
                super::peverify(&exec_path, test_dir);
                super::test_dotnet_executable(&exec_path, test_dir);
            }
        }
    };
}
macro_rules! cargo_test {
    ($test_name:ident) => {
        mod $test_name {

            #[cfg(test)]
            static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            #[test]
            fn cargo_debug() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./cargo_tests/", stringify!($test_name), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                let backend = super::absolute_backend_path();
                let backend = backend.display();
                let linker = super::RUSTC_CODEGEN_CLR_LINKER.display();
                let rustflags = format!("-Z codegen-backend={backend} -C linker={linker}");
                // Compiles the test project
                let out = std::process::Command::new("cargo")
                    .env("RUSTFLAGS", &rustflags)
                    .current_dir(test_dir)
                    .args(["build"])
                    .output()
                    .expect("failed to execute process");
                // panic!("out:{out:?}");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stderr = String::from_utf8(out.stderr.clone())
                        .expect("rustc error contained non-UTF8 characters.");

                    if !stderr.contains("Finished") {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        if !stderr.contains("Finished") {
                            panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                        }
                    }
                }
                drop(lock);
                //let exec_path = concat!("../", stringify!($test_name));
                //test_dotnet_executable(exec_path, test_dir);
            }
            #[test]
            fn cargo_release() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./cargo_tests/", stringify!($test_name), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                let backend = super::absolute_backend_path();
                let backend = backend.display();
                let linker = super::RUSTC_CODEGEN_CLR_LINKER.display();
                let rustflags = format!("-Z codegen-backend={backend} -C linker={linker}");
                // Compiles the test project
                let mut command = std::process::Command::new("cargo");
                command
                    .env("RUSTFLAGS", &rustflags)
                    .current_dir(test_dir)
                    .args([
                        "build",
                        "--release", //"--target",
                                     //"clr64-unknown-clr"
                    ]);
                let out = command.output().expect("failed to execute process");

                // panic!("out:{out:?}");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    if !stderr.contains("Finished") {
                        panic!(
                            "command:{command:?} failed. \n stdout:\n{stdout}\nstderr:\n{stderr}"
                        );
                    }
                }
                drop(lock);
                todo!();
                //let exec_path = concat!("../", stringify!($test_name));
                //test_dotnet_executable(exec_path, test_dir);
            }
        }
    };
}
macro_rules! cargo_test_ignored {
    ($test_name:ident) => {
        mod $test_name {

            #[cfg(test)]
            static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            #[ignore]
            #[test]
            fn cargo_debug() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./cargo_tests/", stringify!($test_name), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                let backend = super::absolute_backend_path();
                let backend = backend.display();
                let linker = super::RUSTC_CODEGEN_CLR_LINKER.display();
                let rustflags = format!("-Z codegen-backend={backend} -C linker={linker}");
                // Compiles the test project
                let out = std::process::Command::new("cargo")
                    .env("RUSTFLAGS", &rustflags)
                    .current_dir(test_dir)
                    .args(["build"])
                    .output()
                    .expect("failed to execute process");
                // panic!("out:{out:?}");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stderr = String::from_utf8(out.stderr.clone())
                        .expect("rustc error contained non-UTF8 characters.");

                    if !stderr.contains("Finished") {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        if !stderr.contains("Finished") {
                            panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                        }
                    }
                }
                drop(lock);
                //let exec_path = concat!("../", stringify!($test_name));
                //test_dotnet_executable(exec_path, test_dir);
            }
            #[ignore]
            #[test]
            fn cargo_release() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./cargo_tests/", stringify!($test_name), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                super::RUSTC_BUILD_STATUS
                    .as_ref()
                    .expect("Could not build rustc!");
                let backend = super::absolute_backend_path();
                let backend = backend.display();
                let linker = super::RUSTC_CODEGEN_CLR_LINKER.display();
                let rustflags = format!("-Z codegen-backend={backend} -C linker={linker}");
                // Compiles the test project
                let mut command = std::process::Command::new("cargo");
                command
                    .env("RUSTFLAGS", &rustflags)
                    .current_dir(test_dir)
                    .args([
                        "build",
                        "--release", //"--target",
                                     //"clr64-unknown-clr"
                    ]);
                let out = command.output().expect("failed to execute process");
                // panic!("out:{out:?}");
                // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                if !out.stderr.is_empty() {
                    let stdout = String::from_utf8(out.stdout)
                        .expect("rustc error contained non-UTF8 characters.");
                    let stderr = String::from_utf8(out.stderr)
                        .expect("rustc error contained non-UTF8 characters.");
                    if !stderr.contains("Finished") || true {
                        panic!(
                            "command:{command:?} failed. \n stdout:\n{stdout}\nstderr:\n{stderr}"
                        );
                    }
                }
                drop(lock);

                //let exec_path = concat!("../", stringify!($test_name));
                //test_dotnet_executable(exec_path, test_dir);
            }
        }
    };
}
#[cfg(debug_assertions)]
fn build_backend() -> Result<(), String> {
    let _out = std::process::Command::new("cargo")
        .args(["build", "--lib"])
        .output()
        .map_err(|err| err.to_string())?;
    std::process::Command::new("cargo")
        .args(["build", "--bin", "linker"])
        .output()
        .expect("could not build the backend");
    /*
    if out.stderr.len() > 0{
        return Err(String::from_utf8(out.stderr).expect("Non UTF8 error message!"));
    }*/
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
#[cfg(test)]
fn absolute_backend_path() -> PathBuf {
    if cfg!(debug_assertions) {
        if cfg!(target_os = "linux") {
            std::fs::canonicalize("target/debug/librustc_codegen_clr.so").unwrap()
        } else if cfg!(target_os = "windows") {
            std::fs::canonicalize("target/debug/librustc_codegen_clr.dll").unwrap()
        } else if cfg!(target_os = "macos") {
            std::fs::canonicalize("target/debug/librustc_codegen_clr.dylib").unwrap()
        } else {
            panic!("Unsupported target OS");
        }
    } else if cfg!(target_os = "linux") {
        std::fs::canonicalize("target/release/librustc_codegen_clr.so").unwrap()
    } else if cfg!(target_os = "windows") {
        std::fs::canonicalize("target/release/librustc_codegen_clr.dll").unwrap()
    } else if cfg!(target_os = "macos") {
        std::fs::canonicalize("target/release/librustc_codegen_clr.dylib").unwrap()
    } else {
        panic!("Unsupported target OS");
    }
}
#[cfg(test)]
fn backend_path() -> &'static str {
    if cfg!(debug_assertions) {
        backend_path_debug()
    } else {
        backend_path_release()
    }
}
#[cfg(test)]
fn backend_path_release() -> &'static str {
    if cfg!(target_os = "linux") {
        "codegen-backend=../../target/release/librustc_codegen_clr.so"
    } else if cfg!(target_os = "windows") {
        "codegen-backend=../../target/release/rustc_codegen_clr.dll"
    } else if cfg!(target_os = "macos") {
        "codegen-backend=../../target/release/librustc_codegen_clr.dylib"
    } else {
        panic!("Unsupported target OS");
    }
}
#[cfg(test)]
fn backend_path_debug() -> &'static str {
    if cfg!(target_os = "linux") {
        "codegen-backend=../../target/debug/librustc_codegen_clr.so"
    } else if cfg!(target_os = "windows") {
        "codegen-backend=../../target/debug/rustc_codegen_clr.dll"
    } else if cfg!(target_os = "macos") {
        "codegen-backend=../../target/debug/librustc_codegen_clr.dylib"
    } else {
        panic!("Unsupported target OS");
    }
}
test_lib! {assign}
test_lib! {binops}
test_lib! {branches}
test_lib! {calls}
test_lib! {casts}
test_lib! {closure}
test_lib! {identity}
test_lib! {libc}

test_lib! {references}
//test_lib! {structs}

test_lib! {types}
test_lib! {recursive}
test_lib! {tuple}

run_test! {arthm,add}
run_test! {types,tuple_structs}
run_test! {arthm,mul}
run_test! {arthm,sub}
run_test! {types,enums}
run_test! {types,nbody}
run_test! {types,structs}
run_test! {types,interop}
run_test! {types,vec}
run_test! {types,string_slice}
run_test! {types,ref_deref}
run_test! {types,slice_ptr_cast}
run_test! {types,slice_index_ref}
run_test! {types,slice}
run_test! {types,statics}
run_test! {std,main}
run_test! {control_flow,cf_for}
run_test! {control_flow,drop}
cargo_test! {hello_world}
cargo_test! {std_hello_world}
cargo_test_ignored! {build_core}
cargo_test_ignored! {build_alloc}
cargo_test_ignored! {build_std}
cargo_test! {benchmarks}
cargo_test! {glam_test}
cargo_test! {fastrand_test}
use lazy_static::*;
lazy_static! {
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
    static ref IS_MONO_PRESENT: bool = std::process::Command::new("mono").output().is_ok();
    static ref IS_PEVERIFY_PRESENT: bool = std::process::Command::new("peverify").output().is_ok();
    static ref IS_DOTNET_PRESENT: bool = std::process::Command::new("dotnet").output().is_ok();
    static ref RUSTC_BUILD_STATUS: Result<(), String> = build_backend();
    static ref RUSTC_CODEGEN_CLR_LINKER:PathBuf = {
        if cfg!(debug_assertions) {
            std::process::Command::new("cargo").args(["build","--bin","linker"]).output().unwrap();
            //TODO: Fix this for other platforms
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                std::fs::canonicalize("target/debug/linker").unwrap()
            } else if cfg!(target_os = "windows") {
                std::fs::canonicalize("target/debug/linker.exe").unwrap()
            }
             else {
                panic!("Unsupported target OS");
            }
        } else {
            std::process::Command::new("cargo").args(["build","--bin","linker","--release"]).output().unwrap();
            //TODO: Fix this for other platforms
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                std::fs::canonicalize("target/release/linker").unwrap()
            } else if cfg!(target_os = "windows") {
                std::fs::canonicalize("target/release/linker.exe").unwrap()
            } else {
                panic!("Unsupported target OS");
            }
        }

    };
}
