#![no_std]
#![no_main]

const EXEC_LIST: u16 = 806;

#[karythra::main]
async fn main() {
    match karythra::ipc::call_service("exec", EXEC_LIST, &[]).await {
        Ok(reply) => {
            if reply.msg_type != 100 || reply.len == 0 {
                karythra::io::print("no instances\n");
                return;
            }
            let count = reply.data[0] as usize;
            if count == 0 {
                karythra::io::print("no instances\n");
                return;
            }
            karythra::io::print("  SLOT  STATE      TID\n");
            for i in 0..count {
                let base = 1 + i * 4;
                let slot = reply.data[base];
                let state = reply.data[base + 1];
                let tid = reply.data[base + 2] as u16 | (reply.data[base + 3] as u16) << 8;
                let mut buf = [b' '; 5];
                let len = karythra::fmt::decimal(slot as u64, &mut buf);
                let pad = 6 - len;
                for _ in 0..pad { karythra::io::print(" "); }
                if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                    karythra::io::print(s);
                }
                let state_name = match state {
                    1 => "  loaded  ",
                    2 => "  running ",
                    3 => "  done    ",
                    _ => "  unknown ",
                };
                karythra::io::print(state_name);
                let mut tbuf = [0u8; 5];
                let tlen = karythra::fmt::decimal(tid as u64, &mut tbuf);
                if let Ok(s) = core::str::from_utf8(&tbuf[..tlen]) {
                    karythra::io::print(s);
                }
                karythra::io::print("\n");
            }
        }
        Err(_) => karythra::io::print("exec service not found\n"),
    }
}
