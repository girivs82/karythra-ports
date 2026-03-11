#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let name = args.trim_ascii();
    if name.is_empty() {
        karythra::io::print("usage: stat <filename>\n");
        return;
    }
    match karythra::fs::stat(name).await {
        Ok(reply) => {
            if reply.msg_type == 100 {
                let n = reply.len as usize;
                if let Ok(s) = core::str::from_utf8(&reply.data[..n]) {
                    karythra::io::print(s);
                }
            } else {
                karythra::io::print("not found: ");
                karythra::io::print(name);
                karythra::io::print("\n");
            }
        }
        Err(_) => karythra::io::print("fs service not found\n"),
    }
}
