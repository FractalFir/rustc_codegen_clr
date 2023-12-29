fn main(){
    let build_env = rustc_codegen_clr::compile_test::cargo_build_env();
    println!("In order to compile cargo crates with `rustc_codegen_clr`, please set the RUSTFLAGS enviorment variable to:");
    println!();
    println!("\"{build_env}\"");
    println!();
    println!("On linux, you may use the following commmand to quickly set the required enviroment variables:");
    println!();
    println!("export RUSTFLAGS=\"{build_env}\"");
    println!();
    println!("On windows(powershell), you may use the following commmand to quickly set the required enviroment variables:");
    println!();
    println!("$Env:RUSTFLAGS = '{build_env}'");
    println!();
    println!("After you are done working with `rustc_codegen_clr` either unset the enviroment variable OR restart your shell(close the command prompt window).");
    println!("On linux, you may use the following commmand to quickly unset the required enviroment variables:");
    println!();
    println!("unset RUSTFLAGS");
    println!();
    println!("Please note that those varaibles may change when the codegen is updated/rebuilt.");
    println!("After each time the codegen is rebuilt, please use this tool again to get updated build enviroment variables.");
    println!();
    println!("If you are using the project, please remember to:");
    println!("1. Update BOTH rustc and the project on a regular basis.");
    println!("2. Report compiler bugs to the maintainers of `rustc_codegen_clr` not the maintainers of the compiler as a whole.");
    println!("  In 99.999% of the cases, the bug is wihin this project and not the compiler.");
}