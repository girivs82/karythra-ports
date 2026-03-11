#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    match karythra::ipc::call_service("entropy", 300, &[]).await {
        Ok(reply) => {
            if reply.msg_type == 100 {
                let n = (reply.len as usize).min(32);
                for i in 0..n {
                    let b = reply.data[i];
                    let hi = b >> 4;
                    let lo = b & 0x0f;
                    let hex = [
                        if hi < 10 { b'0' + hi } else { b'a' + hi - 10 },
                        if lo < 10 { b'0' + lo } else { b'a' + lo - 10 },
                    ];
                    if let Ok(s) = core::str::from_utf8(&hex) {
                        karythra::io::print(s);
                    }
                }
                karythra::io::print("\n");
            } else {
                karythra::io::print("entropy unavailable\n");
            }
        }
        Err(_) => karythra::io::print("entropy service not found\n"),
    }
}
