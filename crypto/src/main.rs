#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let cmd = args.trim_ascii();
    match cmd {
        "rotate" => {
            match karythra::crypto::rotate().await {
                Ok(()) => karythra::io::print("keys rotated\n"),
                Err(_) => karythra::io::print("crypto: rotate failed\n"),
            }
        }
        "" | "status" => {
            match karythra::crypto::status().await {
                Ok(st) => {
                    karythra::io::print("keys: ");
                    karythra::io::print_dec(st.key_count as u64);
                    karythra::io::print("\nactive: ");
                    karythra::io::print(if st.active { "yes" } else { "no" });
                    karythra::io::print("\nroot ready: ");
                    karythra::io::print(if st.root_ready { "yes" } else { "no" });
                    karythra::io::print("\ngeneration: ");
                    karythra::io::print_dec(st.generation as u64);
                    karythra::io::print("\n");
                }
                Err(_) => karythra::io::print("crypto service not found\n"),
            }
        }
        _ => karythra::io::print("usage: crypto [status|rotate]\n"),
    }
}
