#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    let fs_id = match karythra::ipc::lookup("fs") {
        Some(id) => id,
        None => {
            karythra::io::print("ls: fs service not found\n");
            return;
        }
    };
    match karythra::ipc::send(fs_id, 502, &[]) {
        Ok(()) => {
            karythra::ipc::wait().await;
            let mut buf = [0u8; 2048];
            match karythra::ipc::recv_into(&mut buf) {
                Some((_from, 100, len)) => {
                    if len < 4 { return; }
                    let count = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
                    let entry_size = 48 + 4;
                    let mut off = 4;
                    for _ in 0..count {
                        if off + entry_size > len { break; }
                        let name_end = off + 48;
                        let name_bytes = &buf[off..name_end];
                        let name_len = name_bytes.iter().position(|&b| b == 0).unwrap_or(48);
                        if let Ok(name) = core::str::from_utf8(&name_bytes[..name_len]) {
                            let size = u32::from_le_bytes([
                                buf[name_end], buf[name_end + 1],
                                buf[name_end + 2], buf[name_end + 3],
                            ]);
                            karythra::io::print(name);
                            karythra::io::print("  ");
                            karythra::io::print_dec(size as u64);
                            karythra::io::print("\n");
                        }
                        off += entry_size;
                    }
                }
                Some((_, msg_type, _)) => {
                    karythra::io::print("ls: error (type=");
                    karythra::io::print_dec(msg_type as u64);
                    karythra::io::print(")\n");
                }
                None => karythra::io::print("ls: no reply\n"),
            }
        }
        Err(_) => karythra::io::print("ls: send failed\n"),
    }
}
