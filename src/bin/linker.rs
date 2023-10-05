use std::env;
use rustc_codegen_clr::*;
fn main(){
    use std::io::Read;
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let to_link :Vec<_>= args.iter().filter(|arg|arg.contains(".bc")).collect();
    let output = &args[1 + args.iter().position(|arg|arg == "-o").expect("No output file!")];
    let mut final_assembly = assembly::Assembly::empty();
    for asm_path in &to_link{
        let mut asm_file =
        std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
    let mut asm_bytes = Vec::with_capacity(0x100);
    asm_file
        .read_to_end(&mut asm_bytes)
        .expect("ERROR:Could not load the assembly file!");
    let assembly = postcard::from_bytes(&asm_bytes)
        .expect("ERROR:Could not decode the assembly file!");
    final_assembly = final_assembly.join(assembly);
    }
    stdlib::insert_libc(&mut final_assembly);
    use rustc_codegen_clr::assembly_exporter::AssemblyExporter;
        let path = output;
        /*std::fs::create_dir_all(path.parent().expect("Could not get the target directory"))
            .expect("Could not create the target directory!");*/
        rustc_codegen_clr::assembly_exporter::ilasm_exporter::ILASMExporter::export_assembly(
            &final_assembly,
            path.as_ref(),
        );
    //panic!("to_link:{to_link:?} output:{output:?}")
}