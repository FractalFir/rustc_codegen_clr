# rustc_codegen_clr

> [!WARNING]
> This project is still early in its developement. Bugs, crashes and miscompilations are expected. DO NOT USE IT FOR ANYTHING SERIOUS.

`rustc_codegen_clr` is an experimental Rust to .NET compiler backend. It allows the Rust compiler to turn Rust code into .NET assemblies. This translation is very high-level, and preserves things like types,
field/varaible names. 

The project aims to provide a way to easily use Rust libraries in .NET. It comes with a Rust/.NET interop layer, which allows you to easily interact with .NET code from Rust:
```
use mychorizza::*;
fn main(){
    // Alocate a new GC-managed string builder
    let stringBuilder = StringBuilder::empty();
    // You can easily operate on GC-managed types
    mstring.AppendChar('H');
    mstring.AppendChar('i');
    mstring.AppendChar('.');
}
```
The project will also include support for defining .NET classes from Rust. This is currently heavily WIP, and any feedback is appreciated.
```
// Early WIP syntax, subject to change.
dotnet_typedef! {
  class MyClass inherits [Some::External::Assebmly]SomeNamespace::SomeClass{
    virtual fn ToString(_this:MyClass)->MString{
      "I am a class defined in Rust!".into_managed()
    },
  }
}
```
## Current state of the project

The project currently supports most Rust features (except async and proc macros), but it is not bug-free. It can compile a mostly working version of Rust std, but there are many minor bugs make such `std` not 100% functional.

Most compoenets of `std` are about 95% working.

So, you *can* compile a lot of existing Rust code, but it may not necessarily *work*.
### core, std, and alloc uint tests.

| Name | Pass	| Faliure	| Crash \ Timeout| OK precentage
|--------------------|--------|-------|-------|------|
| Core tests |	1635	| 38	| 41	| 95.39% |
| Alloc tests | 	616	|8 |	40 |	92.77% |
| Alloc benches	| 464	| 0	| 0 |	100.00% |
| Test Harness tests |	57 |	0	| 100.00% |
| std tests	| 955| 33 | 17 |	95.02% |
| Core benches	| 490 | 2| | 98.39% |
## FAQ

### Q: What is it?

**A**: *This is a compiler backend for rustc, which targets the `.NET` platform and runtime; this would enable you to use some Rust libraries from C#/F#, with little effort.*

### Q: Is Rust's memory management useless in .NET?

**A**: *Rust code typically uses the stack more than the heap, which can speed up code running within the CLR runtime. Heap-allocated objects are allocated from unmanaged (non-GC) memory and are allocated and freed in the same way as in Rust.*

### Q: Is this useless since I can already load shared libraries from C#?

**A**: *The Rust APIs this codegen exposes to C#/F# code are only slightly easier to use than those exposed by a .so or .dll Rust library. Interop still requires some effort, but the Rust code is bundled with everything else. Types used from C# are guaranteed to be the same as those in C#, preventing mismatch issues. All types can be safely sent between Rust and C#, with exactly the same layout. Additionally, since all Rust code compiled with this codegen can be bundled with C#/F# code, you no longer need to ship different versions of the library for different architectures. Any architecture supported by CLR works out of the box, using the exact same assembly.*

*You can also avoid the cost of switching between code running within and outside the runtime. This cost is not unbearable, but it is not easily eliminated, and reducing it can have safety penalties. In this case, all code runs within the runtime, meaning there is no transition between code running inside and outside the runtime.*

*Compiling Rust to CLR can potentially improve JIT optimization. Since the CLR's JIT now sees all the code, it can make better decisions about optimization, resulting in faster code.*

### Q: Compatibility?

**A**: *`rustc_codegen_clr` is only tested on Linux x86_64, with the CoreCLR runtime (more commonly known as simply the .NET runtime), on .NET 8. It should work on other platforms, but it is not guaranteed.*

### Q: Whata about Mono?
**A** *The support for the Mono runtime is not as good as it could be. Due to not supported features and differences, 128-bit integers and checked 64-bit integer arithmetic are not supported on Mono.*
*Aligned allocators(__rust_alloc) and certain intrinsics are also not supported. I plan to expand support for Mono, but my resources are limited.*

### Q: Are there any issues?

**A**: *While the backend is extensively tested, it is still far from perfect, and there are still many edge cases that may break this backend.*
**A**: *Currently, there are no .NET-specific versions of `std` or .NET specific target triples. This means that you will need separate .NET assemblies for each OS.*


## Licensing

`rustc_codegen_clr` is dual licensed under MIT license or Apache License, Version 2.0.
