#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    match karythra::entropy::random_bytes(32).await {
        Ok((n, data)) => {
            let mut buf = [0u8; 64];
            let len = karythra::fmt::hex_bytes(&data[..n], &mut buf);
            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                karythra::io::print(s);
            }
            karythra::io::print("\n");
        }
        Err(_) => karythra::io::print("entropy unavailable\n"),
    }
}
