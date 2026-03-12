#![no_std]
#![no_main]

const MONITOR_TOP: u16 = 952;
const REPLY_OK: u16 = 100;

#[karythra::main]
async fn main() {
    let monitor = match karythra::ipc::lookup("monitor") {
        Some(id) => id,
        None => {
            karythra::io::print("monitor service not found\n");
            return;
        }
    };

    if karythra::ipc::send(monitor, MONITOR_TOP, &[]).is_err() {
        karythra::io::print("send failed\n");
        return;
    }
    karythra::ipc::wait().await;

    let mut buf = [0u8; 2048];
    let (_, msg_type, len) = match karythra::ipc::recv_into(&mut buf) {
        Some(v) => v,
        None => {
            karythra::io::print("no reply\n");
            return;
        }
    };
    if msg_type != REPLY_OK || len < 32 {
        karythra::io::print("bad reply\n");
        return;
    }

    let num_tasklets = u32_le(&buf[0..4]);
    let total_pages = u32_le(&buf[4..8]);
    let free_pages = u32_le(&buf[8..12]);
    let uptime_ms = u32_le(&buf[12..16]);
    let num_cpus = u32_le(&buf[16..20]);

    let used_pages = total_pages.saturating_sub(free_pages);
    let total_mb = (total_pages as u64 * 4) / 1024;
    let used_mb = (used_pages as u64 * 4) / 1024;

    karythra::io::print("uptime: ");
    print_num(uptime_ms as u64 / 1000);
    karythra::io::print("s  cpus: ");
    print_num(num_cpus as u64);
    karythra::io::print("  memory: ");
    print_num(used_mb);
    karythra::io::print(" MB / ");
    print_num(total_mb);
    karythra::io::print(" MB\n\n");

    karythra::io::print("CPU  BUSY\n");
    let mut off = 32usize;
    for cpu in 0..num_cpus {
        if off + 8 > len { break; }
        let ticks = u64_le(&buf[off..off+8]);
        off += 8;
        karythra::io::print("  ");
        print_num(cpu as u64);
        karythra::io::print("  ");
        print_num(ticks);
        karythra::io::print("\n");
    }

    karythra::io::print("\n ID  PRI  CPU  STATE      PAGES        TICKS  YIELD  PREEMPT\n");
    for _ in 0..num_tasklets {
        if off + 24 > len { break; }
        let tid = u16_le(&buf[off..off+2]);
        let pages = u16_le(&buf[off+2..off+4]);
        let ticks = u64_le(&buf[off+4..off+12]);
        let yields = u32_le(&buf[off+12..off+16]);
        let preempts = u32_le(&buf[off+16..off+20]);
        let state = buf[off+20];
        let cpu = buf[off+21];
        let pri = buf[off+22];
        off += 24;

        print_col(tid as u64, 3);
        print_col(pri as u64, 5);
        print_col(cpu as u64, 5);
        karythra::io::print(state_name(state));
        print_col(pages as u64, 6);
        print_col(ticks, 13);
        print_col(yields as u64, 7);
        print_col(preempts as u64, 8);
        karythra::io::print("\n");
    }
}

fn state_name(s: u8) -> &'static str {
    match s {
        1 => "  ready    ",
        2 => "  running  ",
        3 => "  deferred ",
        _ => "  unknown  ",
    }
}

fn print_num(v: u64) {
    let mut tmp = [0u8; 20];
    let n = karythra::fmt::decimal(v, &mut tmp);
    if let Ok(s) = core::str::from_utf8(&tmp[..n]) {
        karythra::io::print(s);
    }
}

fn print_col(v: u64, width: usize) {
    let mut tmp = [0u8; 20];
    let n = karythra::fmt::decimal(v, &mut tmp);
    let pad = if width > n { width - n } else { 1 };
    for _ in 0..pad {
        karythra::io::print(" ");
    }
    if let Ok(s) = core::str::from_utf8(&tmp[..n]) {
        karythra::io::print(s);
    }
}

fn u16_le(b: &[u8]) -> u16 {
    (b[0] as u16) | ((b[1] as u16) << 8)
}

fn u32_le(b: &[u8]) -> u32 {
    (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16) | ((b[3] as u32) << 24)
}

fn u64_le(b: &[u8]) -> u64 {
    (b[0] as u64)
        | ((b[1] as u64) << 8)
        | ((b[2] as u64) << 16)
        | ((b[3] as u64) << 24)
        | ((b[4] as u64) << 32)
        | ((b[5] as u64) << 40)
        | ((b[6] as u64) << 48)
        | ((b[7] as u64) << 56)
}
