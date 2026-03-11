#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    const NAMES: &[&str] = &[
        "sched", "power", "uart", "fs", "crypto", "entropy",
        "blk", "tpm", "net", "exec", "xfer", "rsh", "iommu",
    ];
    for name in NAMES {
        if let Some(id) = karythra::ipc::lookup(name) {
            karythra::io::print("  ");
            karythra::io::print(name);
            karythra::io::print(" (id=");
            karythra::io::print_dec(id as u64);
            karythra::io::print(")\n");
        }
    }
}
