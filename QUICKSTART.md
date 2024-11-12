# Building the project
## Download the project
Clone the repo using this command:
```
git clone git@github.com:FractalFir/rustc_codegen_clr.git
```
## Instal the newest nightly rust
```
rustup install nightly
```
## Check that the project is complatible with the version of Rust you installed. 
```
cargo check
```
If the project does not compile due to error messages, try updating your rust version.
```
rustup update
```

If this does not solve the problem, open an issue. The project is only compatible with a narrow set of `rustc` versions, and tries to always use the newest nigthly build.
## Install the .NET runtime

This project requires the .NET runtime to be used. You can find [instalation instructions here](https://dotnet.microsoft.com/en-us/download).

After installing .NET, run ` dotnet --info` to confoirm it is installed propely.

## Install `ilasm`

### Windows

`ilasm` comes [installed with Visual Studio](https://learn.microsoft.com/en-us/dotnet/framework/tools/ilasm-exe-il-assembler). So, you propably already have it. If not, install visual stuido. 

### Linux or MacOS

This tool supports both the "Core" and "Mono" flavours of ILASM. While you *can* install / build `ilasm` separeately, installing the [mono runtime](https://www.mono-project.com/download/stable/) is the easiest option.

### Checking the dependencies 

After you installed `dotnet` and `ilasm`, run `./bin/rustflags.rs` to check if you installed `ilasm` and `dotnet` correctly. 

This script uses the experimental `cargo-script` feaure to will check your enviroment, build the project, and print the flags you need to use it.

### Running tests

As one last step, you can run `cargo test ::stable` to check if the codegen runs as expected. This can take a minute or two, but it should ensure the project is working as expected.

# Using the project 

The project is *relatively* simple to use. You will still have to do some setup to get `core` and `std` working, but it is a straigtforward process. 

At the root of your crate, create a directory named `.cargo`. In this directory, create a file named `config.toml`, with the following contents.

```
[build] 
target = "x86_64-unknown-linux-gnu" # Change to the host target.
[unstable]
build-std = ["core","alloc","std","panic_abort"]
```

Then, run the codegen utility `rustflags` again, by running `cargo --bin rustflags --release` **in the directory `rustc_codegen_clr`**.

It should provide you with the commands necesary for enabling the codegen. They should look something like this:

On Linux:
```bash
export RUSTFLAGS="-Z codegen-backend=/home/USER/rustc_codegen_clr/target/release/librustc_codegen_clr.so -C linker=/home/USER/rustc_codegen_clr/target/release/linker -C link-args=--cargo-support "
```
On Windows:
```powershell
$Env:RUSTFLAGS = '-Z codegen-backend=C:\Users\USER\rustc_codegen_clr\target\release\librustc_codegen_clr.dll -C linker=\Users\USER\rustc_codegen_clr\target\release\linker.exe -C link-args=--cargo-support '
```

You can then run this command in your shell session (aka. command prompt window). This command will enable the codegen for that session(command prompt window) only! 
The project makes **no pernament changes** to your instaltation, and simply closing the shell session(command prompt window) will disable it. 

After this, simply run `cargo run` to compile & run your app inside the .NET runtime. Other cargo commands should work to. There may be quite a few error / warning messages dispalyed, but they will not stop compilation.

NOTE: the project currently does not support any `proc-macros`, due to techincal liminations. Supporting them is possible, but it is currently more effort than it is worth.
