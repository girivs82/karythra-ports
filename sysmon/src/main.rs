#![no_std]
#![no_main]

const MONITOR_SYSMON: u16 = 955;
const MAX_TASKLETS: usize = 24;

#[karythra::main]
async fn main() {
    let mut prev_uptime_ms = 0u32;
    let mut prev_ticks = [0u64; MAX_TASKLETS];
    let mut prev_tids = [0u16; MAX_TASKLETS];
    let mut prev_count = 0usize;

    for iter in 0..10u32 {
        let mut buf = [0u8; 2048];
        let len = match karythra::ipc::call_port_service("monitor", MONITOR_SYSMON, &[], &mut buf) {
            Ok(l) => l as usize,
            Err(_) => {
                karythra::io::print("sysmon: monitor not available\n");
                return;
            }
        };
        if len < 48 {
            karythra::io::print("sysmon: bad reply\n");
            return;
        }

        let uptime_ms = u32_le(&buf[0..4]);
        let total_pages = u32_le(&buf[4..8]);
        let free_pages = u32_le(&buf[8..12]);
        let num_cpus = u32_le(&buf[12..16]);
        let num_tasklets = u32_le(&buf[16..20]);
        let ipc_port_full = u32_le(&buf[20..24]);
        let _reserved = u32_le(&buf[24..28]);
        let irq_total = u32_le(&buf[28..32]);
        let steal_att = u32_le(&buf[32..36]);
        let steal_ok = u32_le(&buf[36..40]);
        let tpm = u64_le(&buf[40..48]);

        let used_pages = total_pages.saturating_sub(free_pages);
        let total_mb = (total_pages as u64 * 4) / 1024;
        let used_mb = (used_pages as u64 * 4) / 1024;
        let mem_pct = if total_pages > 0 { (used_pages as u64 * 100) / total_pages as u64 } else { 0 };

        karythra::io::print("\n── sysmon ── ");
        pn(uptime_ms as u64 / 1000);
        karythra::io::print("s  mem=");
        pn(used_mb);
        karythra::io::print("/");
        pn(total_mb);
        karythra::io::print("MB (");
        pn(mem_pct);
        karythra::io::print("%)  ");
        pn(num_cpus as u64);
        karythra::io::print(" CPUs ──\n");

        // Per-CPU: busy% and queue depth
        let delta_ms = if iter > 0 { uptime_ms.saturating_sub(prev_uptime_ms) } else { 0 };
        let mut off = 48usize;

        karythra::io::print("CPU ");
        for cpu in 0..num_cpus as usize {
            if off + 16 > len { break; }
            let _busy_ticks = u64_le(&buf[off..off+8]);
            let queue = u32_le(&buf[off+8..off+12]);
            off += 16;
            karythra::io::print(" ");
            pn(cpu as u64);
            karythra::io::print(":Q");
            pn(queue as u64);
        }
        karythra::io::print("\n");

        karythra::io::print("IPC port_full=");
        pn(ipc_port_full as u64);
        karythra::io::print("  steal=");
        pn(steal_ok as u64);
        karythra::io::print("/");
        pn(steal_att as u64);
        karythra::io::print("  IRQ=");
        pn(irq_total as u64);
        karythra::io::print("\n\n");

        // Per-tasklet table
        karythra::io::print(" TID  CPU%  PRI CPU PIN PAGES STATE      STARVE\n");

        // Parse tasklets into sortable array
        #[derive(Clone, Copy)]
        struct Entry { tid: u16, pages: u16, ticks: u64, yields: u32, preempts: u32,
                       state: u8, cpu: u8, pri: u8, pinned: u8, starve_ms: u64, cpu_pct: u32 }

        let mut entries = [Entry { tid: 0, pages: 0, ticks: 0, yields: 0, preempts: 0,
                                    state: 0, cpu: 0, pri: 0, pinned: 0, starve_ms: 0, cpu_pct: 0 }; MAX_TASKLETS];
        let mut count = 0usize;

        for _ in 0..num_tasklets {
            if off + 32 > len || count >= MAX_TASKLETS { break; }
            let tid = u16_le(&buf[off..off+2]);
            let pages = u16_le(&buf[off+2..off+4]);
            let ticks = u64_le(&buf[off+4..off+12]);
            let yields = u32_le(&buf[off+12..off+16]);
            let preempts = u32_le(&buf[off+16..off+20]);
            let state = buf[off+20];
            let cpu = buf[off+21];
            let pri = buf[off+22];
            let pinned = buf[off+23];
            let starve_ms = u64_le(&buf[off+24..off+32]);
            off += 32;

            let mut cpu_pct = 0u32;
            if iter > 0 && delta_ms > 0 {
                // Find this tid in previous snapshot
                for j in 0..prev_count {
                    if prev_tids[j] == tid {
                        let dt = ticks.saturating_sub(prev_ticks[j]);
                        // ticks are in hardware timestamp units
                        // uptime_ms delta gives us the wall-clock interval
                        let interval_ticks = delta_ms as u64 * tpm;
                        if interval_ticks > 0 {
                            cpu_pct = ((dt * 100) / interval_ticks) as u32;
                        }
                        break;
                    }
                }
            }

            entries[count] = Entry { tid, pages, ticks, yields, preempts, state, cpu, pri, pinned, starve_ms, cpu_pct };
            count += 1;
        }

        // Sort by cpu_pct descending (simple insertion sort)
        for i in 1..count {
            let mut j = i;
            while j > 0 && entries[j].cpu_pct > entries[j-1].cpu_pct {
                let tmp = entries[j];
                entries[j] = entries[j-1];
                entries[j-1] = tmp;
                j -= 1;
            }
        }

        // Print top 12
        let show = count.min(12);
        for i in 0..show {
            let e = &entries[i];
            pc(e.tid as u64, 4);
            pc(e.cpu_pct as u64, 5);
            karythra::io::print("%");
            karythra::io::print("  P");
            pn(e.pri as u64);
            karythra::io::print("  C");
            pn(e.cpu as u64);
            karythra::io::print(if e.pinned != 0 { "  Y " } else { "  - " });
            pc(e.pages as u64, 5);
            karythra::io::print(match e.state { 1 => " ready    ", 2 => " running  ", 3 => " deferred ", _ => " ???      " });
            pc(e.starve_ms, 6);
            karythra::io::print("ms\n");
        }

        // Save for delta next iteration
        prev_uptime_ms = uptime_ms;
        prev_count = count;
        for i in 0..count.min(MAX_TASKLETS) {
            prev_tids[i] = entries[i].tid;
            prev_ticks[i] = entries[i].ticks;
        }

        if iter < 9 {
            karythra::time::sleep(2000).await;
        }
    }
    karythra::io::print("sysmon: done (10 samples)\n");
}

fn pn(v: u64) {
    let mut tmp = [0u8; 20];
    let n = karythra::fmt::decimal(v, &mut tmp);
    if let Ok(s) = core::str::from_utf8(&tmp[..n]) {
        karythra::io::print(s);
    }
}

fn pc(v: u64, width: usize) {
    let mut tmp = [0u8; 20];
    let n = karythra::fmt::decimal(v, &mut tmp);
    let pad = if width > n { width - n } else { 1 };
    for _ in 0..pad { karythra::io::print(" "); }
    if let Ok(s) = core::str::from_utf8(&tmp[..n]) {
        karythra::io::print(s);
    }
}

fn u16_le(b: &[u8]) -> u16 { (b[0] as u16) | ((b[1] as u16) << 8) }
fn u32_le(b: &[u8]) -> u32 { (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16) | ((b[3] as u32) << 24) }
fn u64_le(b: &[u8]) -> u64 {
    (b[0] as u64) | ((b[1] as u64) << 8) | ((b[2] as u64) << 16) | ((b[3] as u64) << 24) |
    ((b[4] as u64) << 32) | ((b[5] as u64) << 40) | ((b[6] as u64) << 48) | ((b[7] as u64) << 56)
}
