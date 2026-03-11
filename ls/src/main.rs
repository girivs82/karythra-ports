#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    match karythra::ipc::call_service("fs", 502, &[]).await {
        Ok(reply) => {
            if reply.msg_type == 100 {
                let n = reply.len as usize;
                if let Ok(s) = core::str::from_utf8(&reply.data[..n]) {
                    karythra::io::print(s);
                }
            } else {
                karythra::io::print("(empty)\n");
            }
        }
        Err(_) => karythra::io::print("fs service not found\n"),
    }
}
