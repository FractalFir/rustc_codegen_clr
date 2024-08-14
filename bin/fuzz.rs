#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
rayon = "1.10.0"
strsim = "0.11.1"
---
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
    rustc_codegen_clr::compile_test::RUSTC_BUILD_STATUS
        .as_ref()
        .expect("Could not build rustc!");
    // Compiles the test project
    let mut cmd = std::process::Command::new("rustc");
    //.env("RUST_TARGET_PATH","../../")
    let rustc_args = rustc_codegen_clr::compile_test::rustc_args();
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
        rustc_codegen_clr::compile_test::test_dotnet_executable(&dotnet_wrapper, test_dir);
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
