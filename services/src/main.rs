#![no_std]
#![no_main]

const MSG_MONITOR_STATUS: u16 = 950;
const MSG_MONITOR_HEALTH: u16 = 951;
const REPLY_OK: u16 = 100;

#[karythra::main]
async fn main() {
    let monitor_id = match karythra::ipc::lookup("monitor") {
        Some(id) => id,
        None => {
            karythra::io::print("monitor not found\n");
            return;
        }
    };

    if let Ok(msg) = karythra::ipc::call(monitor_id, MSG_MONITOR_STATUS, &[]).await {
        if msg.msg_type == REPLY_OK && msg.len >= 24 {
            let d = &msg.data;
            let uptime_ms = u64::from_le_bytes([d[0],d[1],d[2],d[3],d[4],d[5],d[6],d[7]]);
            let free_pages = u32::from_le_bytes([d[8],d[9],d[10],d[11]]);
            let boot_pages = u32::from_le_bytes([d[12],d[13],d[14],d[15]]);
            let services = u32::from_le_bytes([d[16],d[17],d[18],d[19]]);
            let cpus = u32::from_le_bytes([d[20],d[21],d[22],d[23]]);

            let secs = uptime_ms / 1000;
            let mins = secs / 60;
            let hrs = mins / 60;

            karythra::io::print("uptime: ");
            if hrs > 0 {
                karythra::io::print_dec(hrs);
                karythra::io::print("h ");
            }
            if mins > 0 {
                karythra::io::print_dec(mins % 60);
                karythra::io::print("m ");
            }
            karythra::io::print_dec(secs % 60);
            karythra::io::print("s\n");

            karythra::io::print("cpus: ");
            karythra::io::print_dec(cpus as u64);
            karythra::io::print("\n");

            let used = boot_pages - free_pages;
            let used_mb = (used as u64 * 4) / 1024;
            let free_mb = (free_pages as u64 * 4) / 1024;
            karythra::io::print("memory: ");
            karythra::io::print_dec(used_mb);
            karythra::io::print(" MB used, ");
            karythra::io::print_dec(free_mb);
            karythra::io::print(" MB free\n");

            karythra::io::print("services: ");
            karythra::io::print_dec(services as u64);
            karythra::io::print(" registered\n");
        }
    }

    if let Ok(msg) = karythra::ipc::call(monitor_id, MSG_MONITOR_HEALTH, &[]).await {
        if msg.msg_type == REPLY_OK && msg.len >= 4 {
            let d = &msg.data;
            let alive = u16::from_le_bytes([d[0], d[1]]);
            let dead = u16::from_le_bytes([d[2], d[3]]);
            karythra::io::print("health: ");
            karythra::io::print_dec(alive as u64);
            karythra::io::print(" alive");
            if dead > 0 {
                karythra::io::print(", ");
                karythra::io::print_dec(dead as u64);
                karythra::io::print(" dead");
            }
            karythra::io::print("\n");
        }
    }

    const NAMES: &[&str] = &[
        "monitor", "uart", "blk", "fs", "crypto", "entropy",
        "tpm", "net", "exec", "xfer", "rsh", "iommu",
        "gpu", "display", "wm",
    ];
    karythra::io::print("\n");
    for name in NAMES {
        if let Some(id) = karythra::ipc::lookup(name) {
            karythra::io::print("  ");
            karythra::io::print(name);
            karythra::io::print("\tid=");
            karythra::io::print_dec(id as u64);
            karythra::io::print("\n");
        }
    }
}
