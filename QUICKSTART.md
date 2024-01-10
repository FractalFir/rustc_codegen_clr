This guide will show you how to:
1. Build the project
2. Use the project to build a `Hello Wolrd!` example.
3. Show how to debug/catch miscompilations(TODO)
4. Explain some inner workings of the project(TODO)
# Building the project
## Install dependenices
The project has only 2 external dependenices, which ususaly come packaged together.
1. ILASM - necesary for the project to be able to produce .NET assemblies. Both the `mono` and `dotnet` flavors are supported. Should be included with your runtime.
2. The .NET runtime - needed for runing tests and assemblies. Preferably the newest version, install from [here](https://dotnet.microsoft.com/en-us/download). 

The older `mono` runtime does not support 128 bit ints, so they + debug builds will not work.    

## Update `rustc`

The project will ONLY work with the newest, nightly `rustc`. Run

`rustup update`

to get the newest `rustc` version. 

## Switch to nightly rust

The codegen can only be used in nightly rust.

Run

`rustup default nightly`

to switch to nightly rustc.

## Download the project

Run 

`git clone git@github.com:FractalFir/rustc_codegen_clr.git --depth 1` 

to dowload the newest version of the project.
Using `git` makes it easier to update.

## Run `cargo test`

This will  automatically build all the components of the project: the codegen and the linker. It will also enable you to quickly check if everything is working correctly. As of the writing of this guide, 62 tests should pass. This number may go up and 
down as the project develops, but if more than 50 tests pass, the build process succedded. 

# Building a `hello world` example

## Enabling the codegen

Currently, the project must be enabled for each shell sesion(command prompt). This means that your Rust installation is not changed in any way: the project is enabled temporarly, and will be disabled when you close your shell(command prompt window).

To start using the project, run this command: `cargo run --bin rustflags`. This will run a small utility program, which will tell you the proper configuration for your system.

You should see a couple of helpful instructions, and after the phrase:
```
You may use the following command to quickly set the required environment variables:
```
you should see a command, which when typed into your terminal(command prompt) will enable the project. From now, when you run `cargo build` or `cargo run` in that particular shell sesion(window), the codegen will be invoked, and will compile your 
Rust code for .NET. This change will not affect any other shell session(command prompt window), and will be undone as soon as you close the window.

## Creating a `hello world!` example

Run

`cargo new rust_dotnet_example`

to create the example project.

Create a directory named `.cargo` inside `rust_dotnet_example`. Inside `.cargo`, create a file named `config.toml`. This will be used tell cargo to build a .NET version of the standard library.
Open `config.toml` and set its contents to the following:

```
[build]
# Keep it as "x86_64-unknown-linux-gnu", there is currently no .NET-specific target triple.
target = "x86_64-unknown-linux-gnu"
[unstable]
# Will build `core`,`alloc` and `std` for .NET. "panic_abort" disables unwinding.
build-std = ["core","alloc","std","panic_abort"]
```

This example will not use the Rust/C# interop layer, so we will be using unsafe functions.

Set the contents of `main.rs` to:

```rust
extern "C" {
    fn puts(msg: *const u8);
}
fn main() {
    // A heap-allocated string!
    let mut string = String::with_capacity(100);
    string.push('H');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');
    string.push('.');
    string.push('\n');
    string.push('\0');
    unsafe{puts(string.as_ptr())};
    // String literals work too.
    unsafe{puts("Rust + NET = <3!\n\0".as_ptr())};
}
```
## Running the project

*Make sure the project is enabled in this console window.*
On Linux, just type:

`cargo run`

You should see a bunch of error/warning/debug messages appear. They are expected, and most of them serve as reminders about things that could cause issues in the future.

On other unix-es, `cargo run` should just work too. On windows, `cargo run` may(?) work, but I it probably will not. Just search for the `.exe` file within the `target` directory, and run it by typing `dotnet C:/PATH_TO_YOUR_FILE_HERE/FILE.exe`.

## Interop capabilites
Run `cargo add mycorrhiza`. This should add the `mycorrhiza` interop layer to your project. You can replace `main.rs` with:

```rust
fn main() {
    // C#'s String Builder!
    let sb = mycorrhiza::system::text::StringBuilder::empty();
    // We assemble a managed string(C# String)
    sb.append_char('H');
    sb.append_char('e');
    sb.append_char('l');
    sb.append_char('l');
    sb.append_char('o');
    sb.append_char(' ');
    sb.append_char('W');
    sb.append_char('o');
    sb.append_char('r');
    sb.append_char('l');
    sb.append_char('d');
    sb.append_char('!');
    // Calls `StringBuilder.ToString()`
    let mstr = sb.to_mstring();
    // Calls `Console.WriteLine()`
    mycorrhiza::system::console::Console::writeln_string(mstr);
}
```
After you run the project again, you should see `Hello World!` appear on the screen.

This example uses the builtin interop types, but you can call ANY C# method, and store references to ANY C# type.
## Writing your own interop code: Constructors + Calling any C# function
*TODO: Showcase interop*
# Troubleshooting an assebmly
You can set the folowing enviroment variables to aid you in debuging:
`TRACE_STATEMENTS` - prints debug info for each executed MIR statement. Usefull for diagnosing miscompilations. Also disables optimization.
