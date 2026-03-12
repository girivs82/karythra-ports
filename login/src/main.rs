#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let passphrase = args.trim_ascii();
    if passphrase.is_empty() {
        karythra::io::print("usage: login <passphrase>\n");
        return;
    }
    match karythra::fs::session_login(passphrase).await {
        Ok(info) => {
            let console = karythra::api::console_id();
            if console != 0 {
                let _ = karythra::fs::session_inherit(console).await;
            }
            karythra::io::print("session ");
            let mut hex = [0u8; 16];
            let n = karythra::fmt::hex_bytes(&info.session_id, &mut hex);
            if let Ok(s) = core::str::from_utf8(&hex[..n]) {
                karythra::io::print(s);
            }
            karythra::io::print("\n");
        }
        Err(_) => karythra::io::print("login failed\n"),
    }
}
