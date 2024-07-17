use std::collections::HashMap;

use cilly::{
    asm::Assembly,
    utilis::{MemoryUsage, MemoryUsageCounter},
    IString,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let asm = load_asm(std::fs::File::open(path).unwrap());
    let mut usage = Usage::default();
    asm.memory_usage(&mut usage);
    usage.dispaly();
}
fn load_asm(mut file: impl std::io::Read) -> Assembly {
    let mut asm_bytes = Vec::with_capacity(0x100);
    file.read_to_end(&mut asm_bytes)
        .expect("ERROR: Could not load the assembly file!");

    postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!")
}

#[derive(Default)]
struct Usage {
    type_usage: HashMap<IString, (usize, HashMap<IString, usize>)>,
}
impl Usage {
    pub fn dispaly(&self) {
        let mut total: Vec<(_, _)> = self.type_usage.iter().collect();
        total.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
        for (name, (size, fields)) in total {
            println!("{name}:{size}");
        }
    }
}
impl MemoryUsageCounter for Usage {
    fn add_type(&mut self, tpe_name: &str, size: usize) {
        self.type_usage.entry(tpe_name.into()).or_default().0 += size;
    }

    fn add_field(&mut self, tpe_name: &str, field_name: &str, size: usize) {
        *self
            .type_usage
            .entry(tpe_name.into())
            .or_default()
            .1
            .entry(field_name.into())
            .or_default() += size;
    }
}
