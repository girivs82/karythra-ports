#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    match karythra::power::get_status().await {
        Ok(status) => {
            karythra::io::print("policy: ");
            karythra::io::print(status.policy.name());
            karythra::io::print("\ncpus online: ");
            karythra::io::print_dec(status.cpus_online as u64);
            karythra::io::print(" (mask=0x");
            let mut buf = [0u8; 4];
            let len = karythra::fmt::hex(status.cpu_mask as u64, &mut buf);
            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                karythra::io::print(s);
            }
            karythra::io::print(")\n");
        }
        Err(_) => karythra::io::print("power service not found\n"),
    }
}
