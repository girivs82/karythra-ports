#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    match karythra::ipc::call_service("power", 901, &[]).await {
        Ok(reply) => {
            if reply.msg_type == 901 || reply.msg_type == 100 {
                let policy = reply.data[0];
                let cpus = reply.data[1];
                let mask_lo = reply.data[2];
                let mask_hi = reply.data[3];
                let mask = (mask_hi as u16) << 8 | mask_lo as u16;

                karythra::io::print("policy: ");
                karythra::io::print(match policy {
                    0 => "performance",
                    1 => "balanced",
                    2 => "powersave",
                    _ => "unknown",
                });
                karythra::io::print("\ncpus online: ");
                karythra::io::print_dec(cpus as u64);
                karythra::io::print(" (mask=0x");
                let hex = [
                    b"0123456789abcdef"[(mask >> 12) as usize & 0xf],
                    b"0123456789abcdef"[(mask >> 8) as usize & 0xf],
                    b"0123456789abcdef"[(mask >> 4) as usize & 0xf],
                    b"0123456789abcdef"[mask as usize & 0xf],
                ];
                if let Ok(s) = core::str::from_utf8(&hex) {
                    karythra::io::print(s);
                }
                karythra::io::print(")\n");
            } else {
                karythra::io::print("error querying power service\n");
            }
        }
        Err(_) => karythra::io::print("power service not found\n"),
    }
}
