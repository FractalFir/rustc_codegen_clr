fn main() {
    let num_lt = std::hint::black_box(const { inst_partition() });
}

const fn inst_partition() -> fn() {
    partition_hoare_branchy_cyclic
}

fn partition_hoare_branchy_cyclic() {}
