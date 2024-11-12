use std::{
    collections::VecDeque,
    io::{stdin, Read},
    num::NonZeroU32,
    panic::catch_unwind,
    time::Instant,
};

use cilly::{
    c_exporter::CExporter,
    v2::{
        asm::{encoded_stats, Assembly},
        cillyir_exporter::CillyIRExpoter,
        il_exporter::ILExporter,
        Access, CILIter, MethodImpl, MethodRefIdx,
    },
    BasicBlock, CILNode, CILRoot, MethodDef,
};
use fxhash::FxHashSet;

fn main() {
    let mut asm = Assembly::default();
    let mut cmd = String::new();
    let mut script: VecDeque<_> = if let Some(script) = std::env::args().nth(1) {
        use std::io::Read;
        let mut tmp: String = String::default();
        std::fs::File::open(script)
            .unwrap()
            .read_to_string(&mut tmp)
            .unwrap();
        tmp.split('\n').map(|s| s.to_owned()).collect()
    } else {
        VecDeque::default()
    };
    loop {
        cmd.clear();
        if let Some(next_line) = script.pop_front() {
            cmd.push_str(&next_line);
        } else {
            stdin().read_line(&mut cmd).unwrap();
        }

        if cmd.trim() == "exit" {
            return;
        }

        let (stem, body) = match cmd.split_once('(') {
            Some((stem, body)) => (stem, body),
            None => (cmd.as_ref(), ""),
        };
        let body = body.trim().trim_matches(')').trim();
        let stem = stem.trim();
        match stem {
            "typecheck" => asm.typecheck(),
            "simptypes" => {
                let start = std::time::Instant::now();
                let mut simplify_candidates = FxHashSet::default();
                for (id, def) in asm.class_defs() {
                    if def.fields().len() == 1 && def.methods().is_empty() {
                        simplify_candidates.insert(id);
                    }
                }
                eprintln!(
                    "Found {} simplificaiton candiates in {} ns. Total types:{}",
                    simplify_candidates.len(),
                    start.elapsed().as_nanos(),
                    asm.class_defs().len()
                );
            }
            "deadcode" => asm.eliminate_dead_code(),
            "opt" => {
                let mut fuel = asm.fuel_from_env();
                eprintln!("Preparing to optimize the assembly with {fuel:?}");
                let start = std::time::Instant::now();
                asm.opt(&mut fuel);
                if fuel.exchausted() {
                    eprintln!(
                        "Optimization exchausted fuel, finishing early. Took {} ms",
                        start.elapsed().as_millis()
                    );
                } else {
                    eprintln!(
                        "Optimized the assemblty, {fuel:?} remains. Took {} ms",
                        start.elapsed().as_millis()
                    );
                }
            }
            "gc" => asm = asm.clone().link(Assembly::default()),
            "exit" => return,
            "open" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                let mut asm_file =
                    std::fs::File::open(path).expect("ERROR:Could not open the assembly file!");
                let mut asm_bytes = Vec::with_capacity(0x10000);
                asm_file
                    .read_to_end(&mut asm_bytes)
                    .expect("ERROR: Could not load the assembly file!");
                println!("Loading an assembly");
                let loaded_asm = postcard::from_bytes(&asm_bytes).unwrap();
                // Loading an assembly

                if asm.class_defs().is_empty() {
                    asm = loaded_asm;
                } else {
                    println!("Linking an assembly");
                    asm = asm.clone().link(loaded_asm);
                }
                println!("Loaded assembly");
            }
            "save" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                println!("Preparing to save the assembly");
                asm.save_tmp(&mut std::fs::File::create(path).unwrap())
                    .unwrap();
                println!("Saved the assembly");
            }
            "mbyaccess" => {
                // Print all methods with a certain visibility
                let access = body;
                let access = match access {
                    "extern" => Access::Extern,
                    "priv" | "private" => Access::Private,
                    "pub" | "public" => Access::Public,
                    _ => {
                        eprintln!("Unknown accessibility {access:?}");
                        continue;
                    }
                };
                for (id, def) in asm.methods_with(|_, _, def| *def.access() == access) {
                    println!("{name:?} {id:?}", name = &asm[def.name()]);
                }
            }
            "mbyname" => {
                // Print all methods with a certain visibility
                let name = body;
                let Some(iter) = asm.find_methods_matching(name) else {
                    continue;
                };
                for id in iter {
                    println!("{id:?}");
                }
            }
            "toil" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                let start = Instant::now();
                println!("Preparing to export the assembly");
                asm.export(path, ILExporter::new(cilly::v2::IlasmFlavour::Clasic, true));
                println!(
                    "Exported the assembly in {} ms",
                    start.elapsed().as_millis()
                );
            }
            "toc" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                let start = Instant::now();
                println!("Preparing to export the assembly");
                asm.export(path, CExporter::new(false));
                println!(
                    "Exported the assembly in {} ms",
                    start.elapsed().as_millis()
                );
            }
            "tocillyir" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                println!("Preparing to export the assembly");
                asm.export(path, CillyIRExpoter::default())
            }
            "mmakemissing" => {
                let id = parse_id(body, &asm);
                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                asm.modify_methodef(
                    |_, method| *method.implementation_mut() = MethodImpl::Missing,
                    id,
                );
            }
            "mstats" => {
                let id = parse_id(body, &asm);
                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                let def = asm.method_def(id);
                let name = &asm[def.name()];
                let (blocks, locals) = match def.resolved_implementation(&asm) {
                    MethodImpl::MethodBody { blocks, locals } => (blocks, locals),
                    MethodImpl::Extern {
                        lib,
                        preserve_errno,
                    } => {
                        let lib = &asm[*lib];
                        eprintln!(
                            "Extern method {name} is in {lib} preserve_errno:{preserve_errno}"
                        );
                        return;
                    }
                    MethodImpl::AliasFor(_) => panic!("Unresolved method alias"),
                    MethodImpl::Missing => {
                        eprintln!("Missing method {name}");
                        return;
                    }
                };
                eprintln!(
                    "Method {name} has {locals} locals and {blocks} blocks.\n It has {roots} roots and {nodes} elements(roots + nodes).",
                    locals = locals.len(),
                    blocks = blocks.len(),
                    roots = blocks.iter().flat_map(|block|block.iter_roots()).count(),
                    nodes = blocks.iter().flat_map(|block|block.iter_roots()).flat_map(|root|CILIter::new(asm.get_root(root).clone(),&asm)).count(),
                );
            }
            "msetaccess" => {
                let mut body = body.split(',');
                let (id, access) = (parse_id(body.next().unwrap(), &asm), body.next().unwrap());
                let access = match access {
                    "extern" => Access::Extern,
                    "priv" | "private" => Access::Private,
                    "pub" | "public" => Access::Public,
                    _ => {
                        eprintln!("Unknown accessibility {access:?}");
                        continue;
                    }
                };

                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                asm.modify_methodef(|_, method| method.set_access(access), id);
            }
            "" => continue,
            "misolate" => {
                let isolate_id = parse_id(body, &asm);
                let Some(isolate_id) = asm.method_ref_to_def(isolate_id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                misolate(&mut asm, *isolate_id)
            }
            "find_invalid_c" => find_invalid_c(&asm),
            "asmstats" => {
                println!(
                    "methoddefs:{}",
                    asm.method_refs()
                        .iter_keys()
                        .filter_map(|key| asm.method_ref_to_def(key))
                        .count()
                );
                println!("strings:{:?}", encoded_stats(asm.strings()));
            }
            "shorten_strings" => {
                let size_cap: usize = body.parse().unwrap();
                asm.shorten_strings(size_cap)
            }
            _ => eprintln!("unknown command {cmd:?}"),
        }
    }
}
fn find_invalid_c(asm: &Assembly) {
    let mut fail_id = 0;
    for key in asm
        .method_refs()
        .iter_keys()
        .filter_map(|key| asm.method_ref_to_def(key))
    {
        let mut copy = asm.clone();
        misolate(&mut copy, *key);
        if !is_valid_c(&copy, fail_id) {
            fail_id += 1;
            eprintln!("Invalid c code methodid {key:?} fail_id:{fail_id}")
        } else {
            eprintln!("Method ok.")
        }
    }
    println!("Found {} faliures, saved to tmp", fail_id)
}
fn is_valid_c(asm: &Assembly, id: u32) -> bool {
    catch_unwind(|| asm.export(format!("/tmp/test{id}.out"), CExporter::new(false))).is_ok()
}
fn misolate(asm: &mut Assembly, isolate_id: MethodRefIdx) {
    let externs: Vec<_> = asm
        .methods_with(|_, _, def| *def.access() == Access::Extern)
        .map(|(id, _)| id)
        .copied()
        .collect();
    for extern_id in externs {
        asm.modify_methodef(|_, method| method.set_access(Access::Public), extern_id);
    }

    let main_module = asm.main_module();
    let entrypoint = asm.alloc_string("entrypoint");
    let sig = asm.sig([], cilly::Type::Void);
    let fn_ptr = asm.alloc_node(CILNode::LdFtn(isolate_id));
    let roots = [
        asm.alloc_root(CILRoot::Pop(fn_ptr)),
        asm.alloc_root(CILRoot::VoidRet),
    ];
    let implementation = MethodImpl::MethodBody {
        blocks: vec![BasicBlock::new(roots.into(), 0, None)],
        locals: vec![],
    };
    let entrypoint = MethodDef::new(
        Access::Extern,
        main_module,
        entrypoint,
        sig,
        cilly::cilnode::MethodKind::Static,
        implementation,
        vec![],
    );
    asm.new_method(entrypoint);
    asm.eliminate_dead_code();
    // GC
    *asm = asm.clone().link(Assembly::default());
    asm.remove_dead_statics();
}
fn parse_id(id: &str, asm: &Assembly) -> MethodRefIdx {
    if let Ok(id) = id.parse::<u32>() {
        unsafe { MethodRefIdx::from_raw(NonZeroU32::new(id).unwrap()) }
    } else {
        let Some(mut iter) = asm.find_methods_matching(id) else {
            panic!("{id:?} is neithier a method name nor a method id")
        };
        *iter
            .next()
            .unwrap_or_else(|| panic!("{id:?} is neithier a method name nor a method id"))
    }
}
