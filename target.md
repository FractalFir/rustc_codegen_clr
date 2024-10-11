Hello! 

I have a few questions about the process of adding some support for another target / target_os. 

I am currently working on a Rust-to-.NET compiler backend, and I am coming closer and closre to a point at which the project will need some target specific std patches to progress.

Currently, I build a "suroggate" std, which "thinks" it is being built for x86_64 Linux GNU. 

However,  there are a few ugly issues caused by this proccess: 
1. Errno is a bit wonky in .NET: calling any extern function can change the value of errno, even when that function succeds
2. Some libc functions (like fork or pthreads) cause issues(like crashes) in .NET
3. A "surogte" version of std only works on the platform it was designed for, and is not cross platform. If I used .NET APIs, then it could be cross platform. 
4. Each extern function used by a .NET assembly needs some additional data to work(info about how it interacts with errno, and sometimes the signature). So, for each OS I would like to support, I need to input quite a bit of additonal information.
5. Supproting POSIX-stlye enviroment variables(environ) in .NET requires some emulation and a few wierd tricks. 
Keeping those emulated enviroment variables in sync is a bit of a headache, so functions like `set_env` do not always work as expected(the changes are only visible in specific circumstances)
6. Rust uses a bit of a linker hack to get the command line arguments on GNU linux. Emulating this hack requires quite a bit of effort, and compicates the codebase singificantly. 
7. Supporting thread local destructors would, once again, require emulating yet another POSIX APIs. If `std` .NET specific APIs(and .NET threads), I could make the process singificantly easier. 
8. Supporting certain mutithreading APIs(particulary around thread names) is also currently a bit dificult, since I have to emulate the POSIX APIs.

There are also some more issues steming from the lack of a proper .NET target. For example:

.NET does not support 8 and 16 bit atomics yet, so they have to be emulated in a non-compilant way(by using locks). Defining a target would allow me to just tell the compiler that those are not supported.

While I can define a new target withou the upstream support, I can't add .NET support to `std` without copperating with the upstream, so I have been looking at my options.

I have read the [Rust target tier policy](https://doc.rust-lang.org/rustc/target-tier-policy.html) - in particular, the section about tier 3 targets.

Going through all the requirements, I have a few questions regarding them.

1. A tier 3 target must have a designated developer - No problem here, I assume I can just be the designated maintainer
2. Targets must use naming consistent with any existing targets - I have a rough idea about how to name my target `dotnet-core` or `dotnet-core-core9`, altough I am not sure if those are fully consitent?
3. Tier 3 targets may have unusual requirements to build or use - I am not sure about everyting in this point
    3.1. The target must not introduce license incompatibilities - I don't know what exactly does this mean. 
    3.2 Anything added to the Rust repository must be under the standard Rust license (MIT OR Apache-2.0). - No problem here, I my repo is licensed under `MIT OR Apache-2.0`.
    3.3 The target must not cause the Rust tools or libraries built for any other host (even when supporting cross-compilation to the target) to depend on any new dependency less permissive than the Rust licensing policy. -
        my backend depeds on the IL assembler, which is licensed under MIT, and running a compiled assembly requires a .NET runtime(also licensed under MIT). Is MIT alone(so no Apache) considered less premisive than "the Rust licensing policy"?
    3.4 Compiling, linking, and emitting functional binaries, libraries, or other code for the target (whether hosted on the target itself or cross-compiling from another target) must not depend on proprietary (non-FOSS) libraries. - 
        Once again, 