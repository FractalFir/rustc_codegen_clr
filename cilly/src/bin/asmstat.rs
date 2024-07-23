use cilly::v2::asm::Assembly;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let asm = load_asm(std::fs::File::open(path).unwrap());
    asm.memory_info();
}
fn load_asm(mut file: impl std::io::Read) -> Assembly {
    let mut asm_bytes = Vec::with_capacity(0x100);
    file.read_to_end(&mut asm_bytes)
        .expect("ERROR: Could not load the assembly file!");

    postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!")
}
