use std::path::PathBuf;

#[cfg(test)]
fn test_dotnet_executable(file_path: &str, test_dir: &str) -> String {
    use std::io::Write;

    let exec_path = &format!("{file_path}.exe");
    let mut stdout = String::new();
    //println!("exec_path:{exec_path:?}");
    if *IS_DOTNET_PRESENT {
        let config_path = format!("{test_dir}/{file_path}.runtimeconfig.json");
        println!("{config_path:?}");
        let mut file = std::fs::File::create(config_path).unwrap();
        file.write_all(RUNTIME_CONFIG.as_bytes())
            .expect("Could not write runtime config");
        //RUNTIME_CONFIG
        let mut cmd = std::process::Command::new("timeout");
        cmd.arg("-v");
        cmd.arg("5");
        cmd.arg("dotnet");
        cmd.current_dir(test_dir).args([exec_path]);
        #[cfg(target_family = "unix")]
        with_stack_size(&mut cmd, 1024 * 80);
        let out = cmd.output().expect("failed to run test assebmly!");

        let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
        assert!(
            stderr.is_empty(),
            "Test program failed with message {stderr:}"
        );
        stdout = String::from_utf8_lossy(&out.stdout).to_string();
    }
    if *IS_MONO_PRESENT && *crate::config::TEST_WITH_MONO {
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
        assert!(*IS_DOTNET_PRESENT, "Only mono runtime present. Mono does not support all the features required to get Rust code working.");
    }

    assert!(
        (*IS_DOTNET_PRESENT || *IS_MONO_PRESENT),
        "You must have the dotnet runtime installed to run tests."
    );
    stdout
}
#[cfg(test)]
fn test_lib(args: &[&str], test_name: &str) {
    // Ensures the test directory is present
    std::fs::create_dir_all("./test/out").expect("Could not setup the test env");
    // Builds the backend if neceasry
    RUSTC_BUILD_STATUS.as_ref().expect("Could not build rustc!");
    // Compiles the test project
    let mut command = std::process::Command::new("rustc");
    let command = command
        .current_dir("./test/out")
        //.env("RUST_TARGET_PATH","../../")
        .args(args);

    let command = if *IS_MONO_PRESENT {
        // Tell the linker to test AOT
        command.args(["-C", "link-arg=--aot-mode,mono-full"])
    } else {
        command
    };
    let out = command.output().expect("failed to execute process");
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");
        panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
    }
    let test_dll = format!("./{test_name}.dll");
    let out = std::process::Command::new(RUSTC_CODEGEN_CLR_LINKER.display().to_string())
        .current_dir("./test/out")
        .arg("-o")
        .arg(test_dll)
        .arg(format!("./{test_name}.rlib"))
        .output()
        .unwrap();
    //super::peverify(test_dll, "./test/out");
    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");
        panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
    }
}
macro_rules! compare_tests {
    ($prefix:ident,$test_name:ident,$is_stable:ident) => {
        #[cfg(target_os = "linux")]
        mod $test_name {
            mod $is_stable {
                #[cfg(test)]
                #[cfg(test)]
                static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
                #[test]
                fn release() {
                    let lock = COMPILE_LOCK.lock();
                    let mut should_panic = false;
                    let test_dir = concat!("./test/", stringify!($prefix), "/");
                    // Ensures the test directory is present
                    std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                    // Builds the backend if neceasry
                    super::super::RUSTC_BUILD_STATUS
                        .as_ref()
                        .expect("Could not build rustc!");
                    // Compiles the test project
                    let mut cmd = std::process::Command::new("rustc");
                    //.env("RUST_TARGET_PATH","../../")
                    cmd.current_dir(test_dir).args([
                        "-O",
                        "-Z",
                        super::super::backend_path(),
                        "-C",
                        &format!(
                            "linker={}",
                            super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                        ),
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".exe"),
                        "--edition",
                        "2021",
                        //"--target",
                        //"clr64-unknown-clr"
                    ]);
                    let out = cmd.output().expect("failed to execute process");
                    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                    if !out.stderr.is_empty() {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
                        /*if stderr.contains("error"){
                            should_panic = true;
                        }*/
                    }
                    let exec_path = concat!("./", stringify!($test_name));
                    drop(lock);
                    //super::peverify(exec_path, test_dir);
                    eprintln!("Prepating to test with .NET");
                    let dotnet_out = super::super::test_dotnet_executable(exec_path, test_dir);
                    // Compiles the project with native rust
                    let mut cmd = std::process::Command::new("rustc");
                    //.env("RUST_TARGET_PATH","../../")
                    cmd.current_dir(test_dir).args([
                        "-O",
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".a"),
                        "--edition",
                        "2021",
                    ]);
                    let out = cmd.output().expect("failed to execute process");
                    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                    if !out.stderr.is_empty() {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        if stderr.contains("error") || stderr.matches("thread 'rustc'").count() > 1
                        {
                            should_panic = true;
                        }
                        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
                    }
                    let rust_out =
                        std::process::Command::new(concat!("./", stringify!($test_name), ".a"))
                            .current_dir(test_dir)
                            .output()
                            .expect("failed to execute process");
                    let rust_out = String::from_utf8(rust_out.stdout)
                        .expect("rust error contained non-UTF8 characters.");
                    if rust_out != dotnet_out {
                        panic!("rust_out:\n{rust_out}\n\ndotnet_out:\n{dotnet_out}");
                    }

                    if should_panic {
                        panic!("{rust_out}{dotnet_out}");
                    }
                }
                #[test]
                fn debug() {
                    let lock = COMPILE_LOCK.lock();
                    let mut should_panic = false;
                    let test_dir = concat!("./test/", stringify!($prefix), "/");
                    // Ensures the test directory is present
                    std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                    // Builds the backend if neceasry
                    super::super::RUSTC_BUILD_STATUS
                        .as_ref()
                        .expect("Could not build rustc!");
                    // Compiles the test project
                    let mut cmd = std::process::Command::new("rustc");
                    //.env("RUST_TARGET_PATH","../../")
                    cmd.current_dir(test_dir).args([
                        "-Z",
                        super::super::backend_path(),
                        "-C",
                        &format!(
                            "linker={}",
                            super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                        ),
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".exe"),
                        "--edition",
                        "2021",
                        //"--target",
                        //"clr64-unknown-clr"
                    ]);
                    let out = cmd.output().expect("failed to execute process");
                    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                    if !out.stderr.is_empty() {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
                        /*if stderr.contains("error"){
                            should_panic = true;
                        }*/
                    }
                    let exec_path = concat!("./", stringify!($test_name));
                    drop(lock);
                    //super::peverify(exec_path, test_dir);
                    eprintln!("Prepating to test with .NET");
                    let dotnet_out = super::super::test_dotnet_executable(exec_path, test_dir);
                    // Compiles the project with native rust
                    let mut cmd = std::process::Command::new("rustc");
                    //.env("RUST_TARGET_PATH","../../")
                    cmd.current_dir(test_dir).args([
                        "-O",
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".a"),
                        "--edition",
                        "2021",
                    ]);
                    let out = cmd.output().expect("failed to execute process");
                    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                    if !out.stderr.is_empty() {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        if stderr.contains("error") || stderr.matches("thread 'rustc'").count() > 1
                        {
                            should_panic = true;
                        }
                        eprintln!("stdout:\n{stdout}\nstderr:\n{stderr}");
                    }
                    let rust_out =
                        std::process::Command::new(concat!("./", stringify!($test_name), ".a"))
                            .current_dir(test_dir)
                            .output()
                            .expect("failed to execute process");
                    let rust_out = String::from_utf8(rust_out.stdout)
                        .expect("rust error contained non-UTF8 characters.");
                    if rust_out != dotnet_out {
                        panic!("rust_out:\n{rust_out}\n\ndotnet_out:\n{dotnet_out}");
                    }

                    if should_panic {
                        panic!("{rust_out}{dotnet_out}");
                    }
                }
            }
        }
    };
}

macro_rules! test_lib {
    ($test_name:ident,$is_stable:ident) => {
        mod $test_name {
            mod $is_stable {
                #[cfg(test)]
                static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
                #[test]
                fn release() {
                    // Ensures no two compilations run at the same time.
                    let lock = COMPILE_LOCK.lock();
                    super::super::test_lib(
                        &[
                            "-O",
                            "--crate-type=lib",
                            "-Z",
                            super::super::backend_path(),
                            "-C",
                            &format!(
                                "linker={}",
                                super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                            ),
                            concat!("../", stringify!($test_name), ".rs"),
                            "-o",
                            concat!("./", stringify!($test_name), ".rlib"),
                            //"--target",
                            // "clr64-unknown-clr"
                        ],
                        stringify!($test_name),
                    );
                    drop(lock);
                }
                #[test]
                fn debug() {
                    let lock = COMPILE_LOCK.lock();
                    super::super::test_lib(
                        &[
                            "--crate-type=lib",
                            "-Z",
                            super::super::backend_path(),
                            "-C",
                            &format!(
                                "linker={}",
                                super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                            ),
                            concat!("../", stringify!($test_name), ".rs"),
                            "-o",
                            concat!("./", stringify!($test_name), ".rlib"),
                            //"--target",
                            // "clr64-unknown-clr"
                        ],
                        stringify!($test_name),
                    );
                    drop(lock);
                }
            }
        }
    };
}
macro_rules! run_test {
    ($prefix:ident,$test_name:ident,$is_stable:ident) => {
        mod $test_name {
            mod $is_stable {
                #[cfg(test)]
                static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
                #[test]
                fn release() {
                    let lock = COMPILE_LOCK.lock();
                    let test_dir = concat!("./test/", stringify!($prefix), "/");
                    // Ensures the test directory is present
                    std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                    // Builds the backend if neceasry
                    super::super::RUSTC_BUILD_STATUS
                        .as_ref()
                        .expect("Could not build rustc!");
                    // Compiles the test project
                    let mut cmd = std::process::Command::new("rustc");
                    //.env("RUST_TARGET_PATH","../../")
                    cmd.current_dir(test_dir).args([
                        "-O",
                        "-Z",
                        super::super::backend_path(),
                        "-C",
                        &format!(
                            "linker={}",
                            super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                        ),
                        concat!("./", stringify!($test_name), ".rs"),
                        "-o",
                        concat!("./", stringify!($test_name), ".exe"),
                        "--edition",
                        "2021",
                        //"--target",
                        //"clr64-unknown-clr"
                    ]);
                    eprintln!("Command: {cmd:?}");
                    let out = cmd.output().expect("failed to execute process");
                    // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
                    if !out.stderr.is_empty() {
                        let stdout = String::from_utf8(out.stdout)
                            .expect("rustc error contained non-UTF8 characters.");
                        let stderr = String::from_utf8(out.stderr)
                            .expect("rustc error contained non-UTF8 characters.");
                        panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
                    }
                    let exec_path = concat!("./", stringify!($test_name));
                    drop(lock);
                    //super::peverify(exec_path, test_dir);

                    super::super::test_dotnet_executable(exec_path, test_dir);
                }
                #[test]
                fn debug() {
                    let lock = COMPILE_LOCK.lock();
                    let test_dir = concat!("./test/", stringify!($prefix), "/");
                    // Ensures the test directory is present
                    std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                    // Builds the backend if neceasry
                    super::super::RUSTC_BUILD_STATUS
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
                            super::super::backend_path(),
                            "-C",
                            &format!(
                                "linker={}",
                                super::super::RUSTC_CODEGEN_CLR_LINKER.display()
                            ),
                            concat!("./", stringify!($test_name), ".rs"),
                            "-o",
                            &output_path,
                            "--edition",
                            "2021",
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
                    let exec_path = format!("./{test_name}");
                    drop(lock);
                    //super::peverify(&exec_path, test_dir);
                    super::super::test_dotnet_executable(&exec_path, test_dir);
                }
            }
        }
    };
}
macro_rules! cargo_test {
    ($test_name:ident,$is_stable:ident) => {
        mod $test_name { mod $is_stable{

            #[cfg(test)]
            static COMPILE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            #[test]
            fn cargo_debug() {
                let lock = COMPILE_LOCK.lock();
                let test_dir = concat!("./cargo_tests/", stringify!($test_name), "/");
                // Ensures the test directory is present
                std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
                // Builds the backend if neceasry
                let rustflags = super::super::cargo_build_env();
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
                let rustflags = super::super::cargo_build_env();
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
                //let exec_path = concat!("../", stringify!($test_name));
                //test_dotnet_executable(exec_path, test_dir);
            }
        }}
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

                let rustflags = super::cargo_build_env();
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

                let rustflags = super::cargo_build_env();
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
pub fn absolute_backend_path() -> PathBuf {
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
#[cfg(target_family = "unix")]
#[cfg(test)]
fn with_stack_size(cmd: &mut std::process::Command, limit_kb: u64) {
    use ::libc::{rlimit, setrlimit, RLIMIT_STACK};
    use std::os::unix::process::CommandExt;

    unsafe {
        cmd.pre_exec(move || {
            setrlimit(
                RLIMIT_STACK,
                &rlimit {
                    rlim_cur: limit_kb * 1024,
                    rlim_max: limit_kb * 1024,
                },
            );
            Ok(())
        })
    };
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
test_lib! {assign,stable}
test_lib! {binops,stable}
test_lib! {branches,stable}
test_lib! {calls,stable}
test_lib! {casts,stable}
test_lib! {closure,stable}
test_lib! {identity,stable}
test_lib! {libc,stable}
run_test! {types,dst,stable}

test_lib! {references,stable}
//test_lib! {structs}
test_lib! {empty_string_slice,stable}
test_lib! {types,stable}
test_lib! {recursive,stable}
test_lib! {fn_ptr,stable}
test_lib! {tuple,stable}

run_test! {arthm,add,stable}
run_test! {arthm,cmp,stable}
run_test! {arthm,shift,stable}
run_test! {arthm,not,stable}

run_test! {arthm,xor,stable}

run_test! {arthm,mul,stable}
run_test! {arthm,greater_than,stable}
run_test! {arthm,sub,stable}

run_test! {types,tuple_structs,stable}
run_test! {types,enums,stable}
run_test! {types,arr,stable}
run_test! {types,adt_enum,stable}
run_test! {types,nbody,stable}
run_test! {types,structs,stable}
run_test! {types,interop,stable}
run_test! {types,vec,stable}
run_test! {types,subslice,stable}
run_test! {types,string_slice,unstable}
run_test! {types,ref_deref,stable}
run_test! {types,slice_ptr_cast,stable}
run_test! {types,slice_index_ref,unstable}
run_test! {types,slice,stable}
run_test! {types,statics,stable}
run_test! {types,async_types,unstable}
run_test! {types,self_referential_statics,stable}
run_test! {types,int128,stable}

run_test! {std,main,unstable}

run_test! {control_flow,cf_for,stable}
run_test! {control_flow,drop,stable}

run_test! {intrinsics,bswap,stable}

run_test! {intrinsics,assert,unstable}
run_test! {intrinsics,addr_of,stable}
run_test! {intrinsics,malloc,stable}
run_test! {intrinsics,offset_of,unstable}
run_test! {intrinsics,printf,unstable}
run_test! {intrinsics,type_id,stable}
run_test! {intrinsics,ptr_offset_from_unsigned,stable}
run_test! {intrinsics,arith_offset,stable}

run_test! {alloc,abox,stable}

run_test! {fuzz,test0,stable}
run_test! {fuzz,test1,stable}

run_test! {fuzz,fuzz0,unstable}

run_test! {cast,i32_to_usize,stable}
run_test! {cast,i32_to_u64,stable}
run_test! {cast,i16_to_u64,stable}
run_test! {cast,i8_to_u64,stable}


compare_tests! {fuzz,fuzz1,unstable}
compare_tests! {fuzz,fuzz2,stable}
compare_tests! {fuzz,fuzz3,stable}
compare_tests! {fuzz,fuzz4,stable}
compare_tests! {fuzz,fuzz5,stable}
compare_tests! {fuzz,fuzz6,unstable}
compare_tests! {fuzz,fuzz7,unstable}
compare_tests! {fuzz,fuzz8,unstable}
compare_tests! {fuzz,fuzz9,unstable}

compare_tests! {fuzz,fuzz10,unstable}
compare_tests! {fuzz,fuzz11,unstable}
compare_tests! {fuzz,fuzz12,unstable}
compare_tests! {fuzz,fuzz13,unstable}
compare_tests! {fuzz,fuzz14,unstable}
compare_tests! {fuzz,fuzz15,stable}
compare_tests! {fuzz,fuzz16,unstable}
compare_tests! {fuzz,fuzz17,unstable}
compare_tests! {fuzz,fuzz18,unstable}
compare_tests! {fuzz,fuzz19,unstable}

compare_tests! {fuzz,fuzz20,unstable}
compare_tests! {fuzz,fuzz21,unstable}
compare_tests! {fuzz,fuzz22,stable}
compare_tests! {fuzz,fuzz23,unstable}
compare_tests! {fuzz,fuzz24,stable}
compare_tests! {fuzz,fuzz25,stable}
compare_tests! {fuzz,fuzz26,unstable}
compare_tests! {fuzz,fuzz27,unstable}
compare_tests! {fuzz,fuzz28,unstable}
compare_tests! {fuzz,fuzz29,stable}

compare_tests! {fuzz,fuzz30,stable}
compare_tests! {fuzz,fuzz31,unstable}
compare_tests! {fuzz,fuzz32,unstable}
compare_tests! {fuzz,fuzz33,stable}
compare_tests! {fuzz,fuzz34,unstable}
compare_tests! {fuzz,fuzz35,unstable}
compare_tests! {fuzz,fuzz36,unstable}
compare_tests! {fuzz,fuzz37,stable}
compare_tests! {fuzz,fuzz38,unstable}
compare_tests! {fuzz,fuzz39,stable}

compare_tests! {fuzz,fuzz40,unstable}
compare_tests! {fuzz,fuzz41,unstable}
compare_tests! {fuzz,fuzz42,unstable}
compare_tests! {fuzz,fuzz43,unstable}
compare_tests! {fuzz,fuzz44,unstable}
compare_tests! {fuzz,fuzz45,stable}
compare_tests! {fuzz,fuzz46,stable}
compare_tests! {fuzz,fuzz47,unstable}
compare_tests! {fuzz,fuzz48,unstable}
compare_tests! {fuzz,fuzz49,unstable}

compare_tests! {fuzz,fuzz50,unstable}
compare_tests! {fuzz,fuzz51,unstable}
compare_tests! {fuzz,fuzz52,stable}
compare_tests! {fuzz,fuzz53,stable}
compare_tests! {fuzz,fuzz54,unstable}
compare_tests! {fuzz,fuzz55,unstable}
compare_tests! {fuzz,fuzz56,unstable}
compare_tests! {fuzz,fuzz57,unstable}
compare_tests! {fuzz,fuzz58,unstable}
compare_tests! {fuzz,fuzz59,unstable}

compare_tests! {fuzz,fuzz60,stable}
compare_tests! {fuzz,fuzz61,unstable}
compare_tests! {fuzz,fuzz62,stable}
compare_tests! {fuzz,fuzz63,unstable}
compare_tests! {fuzz,fuzz64,unstable}
compare_tests! {fuzz,fuzz65,unstable}
compare_tests! {fuzz,fuzz66,unstable}
compare_tests! {fuzz,fuzz67,unstable}
compare_tests! {fuzz,fuzz68,unstable}
compare_tests! {fuzz,fuzz69,unstable}

compare_tests! {fuzz,fuzz70,stable}
compare_tests! {fuzz,fuzz71,unstable}
compare_tests! {fuzz,fuzz72,unstable}
compare_tests! {fuzz,fuzz73,unstable}
compare_tests! {fuzz,fuzz74,stable}
compare_tests! {fuzz,fuzz75,stable}
compare_tests! {fuzz,fuzz76,unstable}
compare_tests! {fuzz,fuzz77,unstable}
compare_tests! {fuzz,fuzz78,unstable}
compare_tests! {fuzz,fuzz79,unstable}

compare_tests! {fuzz,fuzz80,unstable}
compare_tests! {fuzz,fuzz81,unstable}
compare_tests! {fuzz,fuzz82,stable}
compare_tests! {fuzz,fuzz83,unstable}
compare_tests! {fuzz,fuzz84,stable}
compare_tests! {fuzz,fuzz85,unstable}
compare_tests! {fuzz,fuzz86,stable}
compare_tests! {fuzz,fuzz87,stable}
compare_tests! {fuzz,fuzz88,stable}
compare_tests! {fuzz,fuzz89,unstable}

compare_tests! {fuzz,fuzz90,unstable}
compare_tests! {fuzz,fuzz91,unstable}
compare_tests! {fuzz,fuzz92,unstable}
compare_tests! {fuzz,fuzz93,unstable}
compare_tests! {fuzz,fuzz94,unstable}
compare_tests! {fuzz,fuzz95,unstable}
compare_tests! {fuzz,fuzz96,unstable}
compare_tests! {fuzz,fuzz97,unstable}
compare_tests! {fuzz,fuzz98,unstable}
compare_tests! {fuzz,fuzz99,stable}
compare_tests! {fuzz,fuzz100,unstable}

run_test! {fuzz,fail0,stable}
run_test! {fuzz,fail1,stable}
compare_tests! {fuzz,fail3,unstable}

cargo_test! {hello_world,stable}
cargo_test! {std_hello_world,stable}
cargo_test_ignored! {build_core}
cargo_test_ignored! {build_alloc}
cargo_test_ignored! {build_std}
cargo_test! {benchmarks,stable}
cargo_test! {glam_test,unstable}
cargo_test! {fastrand_test,stable}

use lazy_static::*;
pub fn get_runtime_config() -> &'static str {
    &RUNTIME_CONFIG
}
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
    pub static ref RUSTC_CODEGEN_CLR_LINKER:PathBuf = {
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
pub fn cargo_build_env() -> String {
    RUSTC_BUILD_STATUS.as_ref().expect("Could not build rustc!");
    let backend = absolute_backend_path();
    let backend = backend.display();
    let linker = RUSTC_CODEGEN_CLR_LINKER.display();
    let link_args = "--cargo-support";
    format!("-Z codegen-backend={backend} -C linker={linker} -C link-args={link_args}")
}
pub fn runtime_config() -> String {
    RUNTIME_CONFIG.to_string()
}
