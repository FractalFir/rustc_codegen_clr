# rustc_codegen_clr

> [!WARNING]
> This project is a very early proof-of-concept

## Project progress

The project is still very early in development, but it has made significant progress in recent months. The codegen can now compile a wide range of Rust code to CLR, and it is able to generate code that is efficient and interoperable with C#/F# code. However, there are still some issues with the codegen, such as its inability to understand some Rust optimizations and its tendency to crash when it encounters something not yet supported.

### Functionality

- [X] Basic functions get translated properly.
- [X] Arithmetic operations work
- [X] Most `if`'s work.
- [X] Basic `match` works.
- [X] While loops work.
- [X] Calls
- [X] Basic IL optimization.
- [X] Setting value of a reference
- [X] Getting value of a reference
- [ ] Creating slices from arrays
- [X] Creating arrays
- [ ] Indexing arrays *broken*
- [X] Getting values of fields
- [X] Setting fields
- [X] Pointer dereferecing
- [X] Generics *GATS don't work in some edge cases*
- [X] for loops *with some minor limitations*

### Types

> [!NOTE]
> This section says only if a type can be translated for .NET to understand. This **does not** mean the type is fully usable.

- [X] All integer and float types are supported. Support for math with 128-bit integers is very limited
- [X] References are supported
- [ ] Arrays, slices
- [X] Void type
- [X] Combinations of all of the above.
- [X] Structs
- [X] Enums
- [ ] Tuples
- [ ] Traits *Some, not all*
- [X] iterators

## Basic benchmarks

> [!NOTE]
> Those are benchmarks which put Rust on the worst footing, since they involve no allocations/GC at all. They serve as a baseline to determine the best possible performance.
>
> All tests were run in CoreCLR .NET runtime, version `7.0.11` The host system was `Linux fedora 6.5.5-200.fc38.x86_64`, and the CPU was `13th Gen Intel(R) Core(TM) i5-13500HX`.

`Codegen Optimzations Disabled` means that the code was compiled in release mode, but post-MIR, codegen-internal optimizations were disabled.

### Fibonachi of 10, recursive

| Test Method                                | Avg of 10K runs |
| ------------------------------------------ | --------------- |
| Rust native (release)                      | 100 ns          |
| Rust native (debug)                        | 360 ns          |
| Rust .NET (default optimizations)          | 270 ns          |
| Rust .NET (codegen optimizations disabled) | 330 ns          |
| C# release (pure IL)                       | 250 ns          |
| C# debug (pure IL)                         | 370 ns          |

As you can see, the difference between optimized C# and optimized .NET Rust code is not all that big. It is noticeable(~10%), but I would say it is a pretty good result considering how few optimizations are done right now. With a couple bigger changes coming later down the line, the gap could become non-existent in the future. Since this benchmark is meant to show the worst case scenario, Rust could already outperform C# in a wide range of more memory-intensive scenarios.

**However**, you should take all of those results with a pinch of salt. Since there is currently no way to use "proper" .NET bench marking tools, I am relying on the `Stopwatch` class for time and have no way to control for the behavior of the JIT. It seems to optimize the rust code after enough runs, all while the speed of C# dropped significantly. This is not due to thermal throttling or any other variable I can think of - both tests were run multiple times back-to-back(Rust then C# the Rust then C# again), and the results remain consistent. Such oddities point at issues with the testing setup, but the results can still serve as a rough guide about what kinds of performance can be expected.

| Test Method                       | Avg of 100M runs |
| --------------------------------- | ---------------- |
| Rust native (release)             | 107 ns           |
| Rust .NET (default optimizations) | 252.3 ns         |
| C# release (pure IL)              | 281.66 ns        |

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

**A**: *`rustc_codegen_clr` is only tested on Linux x86_64. It may work on other platforms, but it is not guaranteed.

### Q: Are there any issues?

**A**: *The backend still does not understand some Rust optimizations, and you may need to disable them to allow for compilation*.
**A**: *While testing is more extensive, there are still many edge cases that may break this backend.*
**A**: *The backend crashes whenever it encounters something not supported yet*.

## Licensing

`rustc_codegen_clr` is dual licensed under MIT license or Apache License, Version 2.0.
