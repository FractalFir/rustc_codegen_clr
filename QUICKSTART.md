This guide will show you how to:

1. Build the project
2. Use the project to build a `Hello Wolrd!` example.
3. Show how to debug/catch miscompilations
4. Explain some inner workings of the project

# Building the project
## Install dependencies
The project has only 2 external dependencies, which usually come packaged together.
1. ILASM - necessary for the project to be able to produce .NET assemblies. Both the `mono` and `dotnet` flavors are supported. Should be included with your runtime.
2. The .NET runtime - needed for running tests and assemblies. Preferably the newest version, install from [here](https://dotnet.microsoft.com/en-us/download). 

The older `mono` runtime does not support 128 bit ints, so they + debug builds will not work.    

## Update `rustc`

The project will ONLY work with the newest, nightly `rustc`. Run `rustup update` to get the newest `rustc` version. 

## Switch to nightly rust

The codegen can only be used in nightly rust.

Run `rustup default nightly` to switch to nightly rustc.

## Download the project

Run `git clone git@github.com:FractalFir/rustc_codegen_clr.git --depth 1` to download the newest version of the project.
Using `git` makes it easier to update.

## Run `cargo test`

This will  automatically build all the components of the project: the codegen and the linker. It will also enable you to quickly check if everything is working correctly. As of the writing of this guide, 62 tests should pass. This number may go up and 
down as the project develops, but if more than 50 tests pass, the build process succeeded. 

# Building a `Hello World!` example

## Enabling the codegen

Currently, the project must be enabled for each shell session(command prompt). This means that your Rust installation is not changed in any way: the project is enabled temporarly, and will be disabled when you close your shell(command prompt window).

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

```toml
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
## Note about std and core

While a large chunk of `core` works, most of `std` does not. Using almost any std function will cause the final executable to crash. 

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

*TODO: Showcase custom interop - method,properties, constructors, virtual functions*

# How to debug/catch miscompilations
Quite often, the assembly crated form your code will not work. 
Here are some steps which can help you pinpoint the issue.
## Kinds of errors
### A statement failed to compile
If your executable throws an exception with the message: `Tired to run a statement STATEMENT which failed to compile with error message MESSAGE`, this means that the codegen could not properly compile a MIR statement. 

You can search the compilation logs to get the exact cause.
### Tried to invoke missing method
This message means that the codegen could not compile a whole function. This can be a sign of either: 
1. A missing intrinsic/libc function(function has a short, clear name)
2. A Rust method which used some unsupported type/calling convention(function has a long, mangled name)
3. Call target was not properly resolved(function has a long, mangled name, and is generic)

You can search the compilation logs to get the exact cause.
### Called `panic` or `abort` or a NullReferenceException is thrown
If this occurs only in code produced by `rustc_codegen_clr`, this is a sign of a miscompiled statement(a statement compiled, but not how it should). It is hard to pinpoint the cause of this kind of issue, since it may be far from the source of the
exception.
### InvalidProgramException

The codegen has compiled a function incorrectly, and the runtime can't JIT it. Usually caused by DSTs, I am working on fixing this issue. 

## Stack traces and demangling

When an exception occurs, you will get a stack trace. You can see the mangled name of the function where the exception was thrown. To demangle the name:
1. Replace all occurrences of `\_ds\_`(DolarSign) with $.
2. Replace all occurrences of `\_dd\_`(DoubleDot) with ::.
3. Use rustfilt to demangle the symbol. Example:

`rustfilt _ZN3std3sys6common14small_c_string18run_path_with_cstr17hf87a5f08beda9d2bE` -> `std::sys::common::small_c_string::run_path_with_cstr`

This should enable you to pinpoint the offending function.

## Enviroment varaibles related to debugging.

You can set the following environment variables to aid you in debugging:

`TRACE_STATEMENTS` - prints debug info for each executed MIR statement. Useful for diagnosing miscompilations. Also disables optimization. Set to `1` or `true` to enable.
`INSERT_MIR_DEBUG_COMMENTS` - similar to `TRACE_STATEMENTS`, but the debug info will be only present in the `.il` file.
`ALWAYS_INIT_LOCALS` - Makes all local variables be zero-initialized. Set to `1` or `true` to enable.

After changing the environment variables, you will need to run `cargo clean` and recompile for the changes to take effect.

# The structure and philosophy of the project - Guide for contributors


I will first try to explain why the project is structured the way it is. By showcasing how I came to the current design, I hope to make it easy to understand how everything fits together. This design is not final, and may be adjusted as things 
change.

## Philosophy
### Functional

The project is designed in a functional fashion: most of the functions are pure(take immutable arguments) and will always return the same output for the same input. This is especially true for the MIR-to-CIL translator. I want this process
to be highly predictable and consistent.

This makes local variable allocation a bit trickier(it is implemented using special variants of the CILOp enum), but helps to catch bugs.

An exception to this purely-functional style is `TyCache` - it is taken by a mutable reference,  but only to enable caching of type conversion results. A `TyCache` will always behave the same: empty or full.

## Where to start looking

### Understanding CIL

Understanding the project does require having a very basic familiarity with CIL. It is stack - based, which means that all the operations occur on an imaginary stack. For example, look at this method:

C#
```csharp
int Add(int a,int b){
    return a + b;
}
```
CIL
```cil
.method static int32 Add(int32 a,int32 b){
    // Load argument 0 onto the imaginary stack
    ldarg.0
    // Load argument 1 onto the imaginary stack
    ldarg.1
    // Add the top 2 values form the imagnary stack, and push the result.
    add
    // Return the top value from the stack
    ret
}
```
You can take a look at [this website](https://sharplab.io/). It allows you to see how C# code translates to CIL, which should help you understand CIL better.

## Simplest case: `binop.rs`

A good place to start looking is the `binop.rs` file. It shows how to load a MIR operand on the stack, and how to preform an arithmetic operation on it. It also shows how the project deals with types.

## The main MIR handling code: `statement.rs`

`statement.rs` is the function which takes a MIR statement, and calls the relevant functions to translate MIR. `rvalue.rs` is used to evaluate the right hand side of a statement.

## Any questions?

If you have any questions, feel free to ask them on the github discussions page. I will do my best to try and answer your questions about the project.
