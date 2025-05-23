#!/usr/bin/env -S cargo +nightly -Zscript
mod build_backend;
use crate::build_backend::ilasm_check;
fn main() {
    let build_env = crate::build_backend::cargo_build_env();
    let print_raw_env = std::env::args().any(|arg| arg == "--print_raw_env");
    let setup_command = std::env::args().any(|arg| arg == "--setup_command");
    if print_raw_env {
        println!("\"{build_env}\"");
        return;
    }
    if setup_command {
        #[cfg(target_family = "unix")]
        println!("export RUSTFLAGS=\"{build_env}\"");
        #[cfg(target_family = "windows")]
        println!("$Env:RUSTFLAGS = '{build_env}'");
        return;
    }
    println!("Welcome to the `rustc_codegen_clr` environment setup helper!");
    println!("This tool will help you use the codegen to compile Rust projects.");
    println!();
    println!("NOTE: if you are only interested in the C side of the project, set the enviroment variable C_MODE to 1, and run this tool again.");
    println!("Doing dependency checks...");
    if !std::env::var("C_MODE").is_ok() {
        ilasm_check();
    }
    println!("Dependency checks succeeded.");
    println!();
    println!("WARNING: Please note, the project is currently in the early stages of development.  Bugs, crashes and miscompilations will occur.");
    println!("Currently, there is no way to permanently install the codegen. It is enabled on a per-shell-session basis (enabled for your particular command prompt window).");
    println!();
    println!("In order to compile cargo crates with `rustc_codegen_clr`, please set the RUSTFLAGS environment variable to:");
    println!();
    println!();
    println!("\"{build_env}\"");
    println!();
    if std::env::var("C_MODE").is_ok() {
        println!("Additonally, set `C_MODE` to 1");
    }
    println!();
    #[cfg(target_family = "unix")]
    {
        println!(
            "You may use the following command to quickly set the required environment variables:"
        );
        println!();
        println!();
        println!("export RUSTFLAGS=\"{build_env}\"");
        if std::env::var("C_MODE").is_ok() {
            println!("export C_MODE=1");
        }
        println!();
        println!();
    }
    #[cfg(target_family = "windows")]
    {
        println!(
            "You may use the following command to quickly set the required environment variables:"
        );
        println!();
        println!();
        println!("$Env:RUSTFLAGS = '{build_env}'");
        println!();
        println!();
    }
    println!("After you are done working with `rustc_codegen_clr` either unset the environment variable OR restart your shell (close the command prompt window).");
    #[cfg(target_family = "unix")]
    {
        println!("You may use the following command to quickly unset the required environment variables:");
        println!();
        println!();
        println!("unset RUSTFLAGS");
        println!();
        println!();
    }
    #[cfg(target_family = "windows")]
    {
        println!();
    }
    println!("Please note that those variables may change when the codegen is updated/rebuilt.");
    println!("After each time the codegen is rebuilt, please use this tool again to get updated build environment variables.");
    println!();
    println!("If you are using the project, please remember to:");
    println!("1. Update BOTH rustc and the project on a regular basis.");
    println!("2. Report compiler bugs to the maintainers of `rustc_codegen_clr`, and not the maintainers of the Rust compiler as a whole.");
    println!(
        "  In 99.999% of the cases, the bug is within this project and not the Rust compiler."
    );
    println!();
    #[cfg(target_family = "unix")]
    {
        println!("############# Testing with the Rust compiler #####################");
        println!("Testing with the Rust compiler requires:");
        println!("1. running `cargo test ::stable`(builds & tests the backend)");
        println!("2. running ./setup_rustc_fork.sh to download rustc.");
        println!("3. Setting the following environment variables:");
        println!("# For .NET");
        println!("export RUSTFLAGS_NOT_BOOTSTRAP=\"{build_env} -C link-args=--cargo-support\"");
        println!("# For C");
        println!("export C_MODE=1");
        println!("export RUSTFLAGS_NOT_BOOTSTRAP=\"{build_env} -Cpanic=abort -Zpanic_abort_tests -C link-args=--cargo-support\"");
        println!("Go to the `rustc` directory, run `./configure` to setup the compiler.");
        println!("./x test core --stage 0 -j 10");
    }
}
