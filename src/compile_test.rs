#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_dotnet_executable(exec_path: &str, test_dir: &str) {
    // Execute the test assembly
    let out = std::process::Command::new("mono")
        .current_dir(test_dir)
        .args([exec_path])
        .output()
        .expect("failed to run test assebmly!");
    let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
    if !stderr.is_empty() {
        panic!("Test program failed with message {stderr:}");
    }
}
//TODO: While we can ensure all exec_path's come from the test runner, it is also very important to ensure this:
//1. Always executes test
//2. cannot be used to run any arbitray executable not produced by the compiler backend. This is not an issue when using mono, since all of our executables must be .NET assemblies to be executed.
// This is most likely not an issue at all, I just prefer being unecesarly paranoid over shipping broken code.
// Idealy, we would prefer a sanboxed enviroment over this for all targets, but setting it up may require some more effort.
#[cfg(target_os = "windows")]
fn test_dotnet_executable(exec_path: &str, test_dir: &str) {
    todo!("Executing test assemblies on windows is not yet supported, since I am not sure if this is the right way to go about it. Comment out this line if you want to renable this kind of tests.");
    // Execute the test assembly
    let out = std::process::Command::new(exec_path)
        .current_dir(test_dir)
        .output()
        .expect("failed to run test assebmly!");
    let stderr = String::from_utf8(out.stderr).expect("Stdout is not UTF8 String!");
    if !stderr.is_empty() {
        panic!("Test program failed with message {stderr:}");
    }
}
macro_rules! test_lib {
    ($test_name:ident) => {
        #[test]
        fn $test_name() {
            // Ensures the test directory is present
            std::fs::create_dir_all("./test/out").expect("Could not setup the test env");
            // Builds the backend if neceasry
            std::process::Command::new("cargo")
                .args(["build"])
                .output()
                .expect("could not build the backend");
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
            std::process::Command::new("cargo")
                .args(["build"])
                .output()
                .expect("could not build the backend");
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
            let exec_path = concat!("../", stringify!($test_name), ".exe");
            test_dotnet_executable(exec_path, test_dir);
        }
    };
}
#[cfg(test)]
fn backend_path() -> &'static str {
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
test_lib! {structs}

test_lib! {types}

run_test! {arthm,add}
run_test! {types,enums}
