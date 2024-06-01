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
#[dotnet_typedef]
struct Test{
    inherits:System::Object,
    count:i32,
    #[fnimpl(Test_ToString)]
    ToString: fn(Self)->System::String,
    #[fnimpl(Test_GetCount)]
    GetCount: fn(Self)->System::String,
    #[fnimpl(Test_SayHello)]
    SayHello: fn(),
}
```
## Current state of the project

The project currently supports most Rust features (except async), but it is not bug-free. It can compile a partially working version of Rust std, but the many minor bugs make such `std` highly unstable.

So, you *can* compile a lot of existing Rust code, but it may not necessarily *work*.
## Basic benchmarks

> [!NOTE]
> Those are benchmarks which put Rust on the worst footing, since they involve no allocations/GC at all. They serve as a baseline to determine the best possible performance.
>
> All tests were run in CoreCLR .NET runtime, version `7.0.11` The host system was `Linux fedora 6.5.5-200.fc38.x86_64`, and the CPU was `13th Gen Intel(R) Core(TM) i5-13500HX`.

`Codegen Optimizations Disabled` means that the code was compiled in release mode, but post-MIR, codegen-internal optimizations were disabled.

### Fibonacci of 10, recursive

| Test Method                                | Avg of 10K runs |
| ------------------------------------------ | --------------- |
| Rust native (release)                      | 100 ns          |
| Rust native (debug)                        | 360 ns          |
| Rust .NET (default optimizations)          | 240 ns          |
| Rust .NET (codegen optimizations disabled) | 330 ns          |
| C# release (pure IL)                       | 230 ns          |
| C# debug (pure IL)                         | 370 ns          |

As you can see, the difference between optimized C# and optimized .NET Rust code is not all that big. It is noticeable(~10%), but I would say it is a pretty good result considering how few optimizations are done right now. With a couple of bigger changes coming further down the line, the gap could become non-existent in the future. Since this benchmark is meant to show the worst case scenario, Rust could already outperform C# in a wide range of more memory-intensive scenarios.

**However**, you should take all of those results with a pinch of salt. Since there is currently no way to use "proper" .NET benchmarking tools, I am relying on the `Stopwatch` class for time measurements and have no way to control the behavior of the JIT. 

| Test Method                       | Avg of 100M runs |
| --------------------------------- | ---------------- |
| Rust native (release)             | 107 ns           |
| Rust .NET (default optimizations) | 240 ns           |
| C# release (pure IL)              | 220 ns           |

## FAQ

### Q: What is it?

**A**: *This is a compiler backend for rustc, which targets the `.NET` platform and runtime; this would enable you to use some Rust libraries from C#/F#, with little effort.*

### Q: Is Rust's memory management useless in .NET?

**A**: *Rust code typically uses the stack more than the heap, which can speed up code running within the CLR runtime. Heap-allocated objects are allocated from unmanaged (non-GC) memory and are allocated and freed in the same way as in Rust.*

### Q: Is this useless since I can already load shared libraries from C#?

**A**: *The Rust APIs this codegen exposes to C#/F# code are only slightly easier to use than those exposed by a .so or .dll Rust library. Interop still requires some effort, but the Rust code is bundled with everything else. Types used from C# are guaranteed to be the same as those in C#, preventing mismatch issues. All types can be safely sent between Rust and C#, with exactly the same layout. Additionally, since all Rust code compiled with this codegen can be bundled with C#/F# code, you no longer need to ship different versions of the library for different architectures. Any architecture supported by CLR works out of the box, without the exact same binary.*

*You can also avoid the cost of switching between code running within and outside the runtime. This cost is not unbearable, but it is not easily eliminated, and reducing it can have safety penalties. In this case, all code runs within the runtime, meaning there is no transition between code running inside and outside the runtime.*

*Compiling Rust to CLR can potentially improve JIT optimization. Since the CLR's JIT now sees all the code, it can make better decisions about optimization, resulting in faster code.*

### Q: Compatibility?

**A**: *`rustc_codegen_clr` is only tested on Linux x86_64, with the Mono and CoreCLR (more commonly known as simply the .NET runtime). It should work on other platforms, but it is not guaranteed.

**A** The support for the Mono runtime is not as good as it could be. Due to not supported features and differences, 128-bit integers and checked 64-bit integer arithmetic are not supported on Mono.

### Q: Are there any issues?

**A**: *While the backend is extensively tested, it is still far from perfect, and there are still many edge cases that may break this backend.*
**A**: *Currently, there are no .NET-specific versions of `std` or .NET specific target triples. This means that you will need separate .NET assemblies for each OS.*


## Licensing

`rustc_codegen_clr` is dual licensed under MIT license or Apache License, Version 2.0.
