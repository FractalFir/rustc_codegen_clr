use rustc_codegen_clr::{
    assembly::Assembly,
    call,
    cil::CallSite,
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_i32, conv_usize,
    function_sig::FnSig,
    ldc_i32, ldc_u32,
    r#type::{DotnetTypeRef, Type},
};
fn mstring_to_utf8ptr(mstring: CILNode) -> CILNode {
    call!(
        CallSite::new_extern(
            DotnetTypeRef::marshal(),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new(&[DotnetTypeRef::string_type().into()], &Type::ISize),
            true
        ),
        [mstring]
    )
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
/// `std::sys::pal::unix::args::imp::really_init` relays on some funky dynamic linkeker stuff(a function is called *as the program is loaded*) - this patch just emulates that.
fn hijack_arg_init(asm: &mut Assembly) {
    // find std::sys::pal::unix::args::imp::really_init

    use std::num::NonZeroU8;

    use rustc_codegen_clr::{eq, lt, size_of};
    //System.Runtime.InteropServices.Marshal.StringToCoTaskMemUTF8

    let mut really_init = None;
    for site in asm.call_sites() {
        if site.class().is_some() {
            continue;
        };
        let name = site.name();
        if name.contains("really_init")
            && name.contains("args")
            && name.contains("unix")
            && name.contains("pal")
            && name.contains("sys")
            && name.contains("std")
        {
            if let Some(really_init) = really_init {
                assert_eq!(
                    really_init, site,
                    "ERROR: Two arg initialziers found! really_init:{really_init:?} site:{site:?}"
                );
            }
            really_init = Some(site);
        }
    }
    // Patch the cctor to call it:
    let Some(cctor) = asm.cctor_mut() else {
        println!("WARNING: could not patch the arg initializer, because no static initializer is present!");
        return;
    };
    // Allocate the variables necesarry for initializng args.
    let argc = u32::try_from(cctor.add_local(Type::I32, Some("argc".into()))).unwrap();
    let argv = u32::try_from(cctor.add_local(
        Type::Ptr(Type::Ptr(Type::U8.into()).into()),
        Some("argv".into()),
    ))
    .unwrap();
    let managed_args = u32::try_from(cctor.add_local(
        Type::ManagedArray {
            element: Box::new(DotnetTypeRef::string_type().into()),
            dims: NonZeroU8::new(1).unwrap(),
        },
        Some("managed_args".into()),
    ))
    .unwrap();
    let arg_idx = u32::try_from(cctor.add_local(Type::I32, Some("arg_idx".into()))).unwrap();
    // Get managed args
    let margs_init = CILRoot::STLoc {
        local: managed_args,
        tree: call!(
            CallSite::new_extern(
                DotnetTypeRef::enviroment(),
                "GetCommandLineArgs".into(),
                FnSig::new(
                    &[],
                    &Type::ManagedArray {
                        element: Box::new(DotnetTypeRef::string_type().into()),
                        dims: NonZeroU8::new(1).unwrap()
                    }
                ),
                true
            ),
            []
        ),
    };
    // Calculate argc
    let argc_init = CILRoot::STLoc {
        local: argc,
        tree: conv_i32!(CILNode::LDLen {
            arr: CILNode::LDLoc(managed_args).into()
        }),
    };
    // Alloc argv
    let argv_alloc = CILRoot::STLoc {
        local: argv,
        tree: call!(
            CallSite::alloc(),
            [conv_usize!(CILNode::LDLoc(argc)), conv_usize!(ldc_u32!(8))]
        ),
    };
    // Create the block which allocates argv and calculates argc.
    let start_bb = cctor.new_bb();
    let mut blocks = cctor.blocks_mut();
    // Remove the return and jump to the patch block
    let ret = blocks[(start_bb - 1) as usize].trees_mut().pop();
    assert_eq!(
        ret,
        Some(CILRoot::VoidRet.into()),
        "ERROR: static constructor did not end with a return!"
    );
    blocks[(start_bb - 1) as usize].trees_mut().push(
        CILRoot::GoTo {
            target: start_bb,
            sub_target: 0,
        }
        .into(),
    );
    // Fill up the start block
    let start_block = &mut blocks[start_bb as usize];
    start_block.trees_mut().push(margs_init.into());
    start_block.trees_mut().push(argc_init.into());
    start_block.trees_mut().push(argv_alloc.into());
    // Init arg_idx to 0
    start_block.trees_mut().push(
        CILRoot::STLoc {
            local: arg_idx,
            tree: ldc_i32!(0),
        }
        .into(),
    );
    start_block.trees_mut().push(
        CILRoot::GoTo {
            target: start_bb + 1,
            sub_target: 0,
        }
        .into(),
    );
    drop(blocks);
    // Set-up the arg convertion loop
    let loop_bb = cctor.new_bb();
    let mut blocks = cctor.blocks_mut();
    let loop_block = &mut blocks[loop_bb as usize];
    // Load nth argument
    let arg_nth = CILNode::LDElelemRef {
        arr: CILNode::LDLoc(managed_args).into(),
        idx: CILNode::LDLoc(arg_idx).into(),
    };
    // Convert the nth managed argument to UTF16
    let uarg = mstring_to_utf8ptr(arg_nth);
    // Store the converted arg at idx+1
    loop_block.trees_mut().push(
        CILRoot::STIndISize(
            CILNode::LDLoc(argv) + conv_usize!(size_of!(Type::ISize) * CILNode::LDLoc(arg_idx)),
            uarg,
        )
        .into(),
    );
    // Incr the arg_idx
    loop_block.trees_mut().push(
        CILRoot::STLoc {
            local: arg_idx,
            tree: CILNode::LDLoc(arg_idx) + ldc_i32!(1),
        }
        .into(),
    );
    //If no args left, jump to exit
    loop_block.trees_mut().push(
        CILRoot::BTrue {
            target: loop_bb + 1,
            sub_target: 0,
            ops: eq!(
                lt!(
                    CILNode::LDLoc(arg_idx),
                    conv_i32!(CILNode::LDLen {
                        arr: CILNode::LDLoc(managed_args).into()
                    })
                ),
                ldc_i32!(0)
            ),
        }
        .into(),
    );
    //If some args left, jump back to loop head!
    loop_block.trees_mut().push(
        CILRoot::GoTo {
            target: loop_bb,
            sub_target: 0,
        }
        .into(),
    );
    drop(blocks);
    // Final bb, which calls the initialzer
    let final_bb = cctor.new_bb();
    let mut blocks = cctor.blocks_mut();
    let final_block = &mut blocks[final_bb as usize];
    if let Some(really_init) = really_init {
        final_block.trees_mut().push(
            CILRoot::Call {
                site: really_init,
                args: [CILNode::LDLoc(argc), CILNode::LDLoc(argv)].into(),
            }
            .into(),
        );
    } else {
        println!("WARNING: could not patch the arg initializer!");
    }
    println!("Propely patched `std::sys::pal::unix::args::imp::really_init`");
    // Re-add the return we removed before.
    final_block.trees_mut().push(CILRoot::VoidRet.into());
}
#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn hijack_arg_init(asm: &mut Assembly) {
    eprintln!("WARNING: Can't patch `std::env::args`, since this target not fully supported!")
}
pub fn patch_all(asm: &mut Assembly) {
    println!("Applying patches to the resulting assembly...");
    hijack_arg_init(asm);
}
/*
public class C {
    static unsafe void really_init(char** argv,int argc){

    }
    public unsafe void M() {
        string[] args = Environment.GetCommandLineArgs();
        string exec_path = System.Reflection.Assembly.GetEntryAssembly().Location;
        int argc = args.Length + 1;
        char** argv = (char**)System.Runtime.InteropServices.NativeMemory.AlignedAlloc((nuint)(argc*sizeof(char*)),8);
        for(int idx = 0; idx < args.Length; idx++){
            argv[idx] = (char*)System.Runtime.InteropServices.Marshal.StringToCoTaskMemUTF8(args[idx]);
        }
        really_init(argv,argc);
    }
}
*/
