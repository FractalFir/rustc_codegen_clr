
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
    if !(*IS_DOTNET_PRESENT || *IS_MONO_PRESENT){
        panic!("You must have either mono or dotnet runtime installed to run tests.");
    }
}
macro_rules! test_lib {
    ($test_name:ident) => {
        #[test]
        fn $test_name() {
            // Ensures the test directory is present
            std::fs::create_dir_all("./test/out").expect("Could not setup the test env");
            // Builds the backend if neceasry
            RUSTC_BUILD_STATUS.as_ref().expect("Could not build rustc!");
            // Compiles the test project
            let out = std::process::Command::new("rustc")
                .current_dir("./test/out")
                //.env("RUST_TARGET_PATH","../../")
                .args([
                    "-O",
                    "--crate-type=lib",
                    "-Z",
                    backend_path(),
                    concat!("../", stringify!($test_name), ".rs"),
                    "-o",
                    concat!("./", stringify!($test_name), ".dll"),
                    //"--target",
                    // "clr64-unknown-clr"
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
        }
    };
}
macro_rules! run_test {
    ($prefix:ident,$test_name:ident) => {
        #[test]
        fn $test_name() {
            let test_dir = concat!("./test/", stringify!($prefix), "/");
            // Ensures the test directory is present
            std::fs::create_dir_all(test_dir).expect("Could not setup the test env");
            // Builds the backend if neceasry
            RUSTC_BUILD_STATUS.as_ref().expect("Could not build rustc!");
            // Compiles the test project
            let out = std::process::Command::new("rustc")
                //.env("RUST_TARGET_PATH","../../")
                .current_dir(test_dir)
                .args([
                    "-O",
                    "-Z",
                    backend_path(),
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
            test_dotnet_executable(exec_path, test_dir);
        }
    };
}
#[cfg(debug_assertions)]
fn build_backend() ->Result<(),String>{
    let out = std::process::Command::new("cargo")
        .args(["build"])
        .output().map_err(|err| err.to_string())?;
    /*
    if out.stderr.len() > 0{
        return Err(String::from_utf8(out.stderr).expect("Non UTF8 error message!"));
    }*/
    Ok(())
}
#[cfg(not(debug_assertions))]
fn build_backend() {
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .expect("could not build the backend");
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
test_lib! {binops}
test_lib! {branches}
test_lib! {calls}
test_lib! {casts}
test_lib! {identity}
test_lib! {libc}
test_lib! {nbody}
test_lib! {references}
//test_lib! {structs}

test_lib! {types}

run_test! {arthm,add}
run_test! {types,enums}
run_test! {types,structs}
run_test! {types,vec}
run_test! {std,main}
use lazy_static::*;
lazy_static! {
    static ref RUNTIME_CONFIG: String = {
        let info = std::process::Command::new("dotnet")
            .arg("--info")
            .output()
            .expect("Could not run `dotnet --info`");
        if info.stderr.len() > 0 {
            let stderr = std::str::from_utf8(&info.stderr).expect("Error message not utf8");
            panic!("dotnet --info panicked with {stderr}")
        }
        let info = std::str::from_utf8(&info.stdout).expect("Error message not utf8");
        let version_start = info.find("Version:").unwrap();
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
    static ref IS_MONO_PRESENT: bool = std::process::Command::new("mono").output().is_ok() ;
    static ref IS_DOTNET_PRESENT: bool = std::process::Command::new("dotnet").output().is_ok() ;
    static ref RUSTC_BUILD_STATUS: Result<(),String> = build_backend();
}
