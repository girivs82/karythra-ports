#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    let mut entries = [karythra::fs::FileEntry {
        name: [0u8; 48],
        name_len: 0,
        size: 0,
    }; 32];
    match karythra::fs::list(&mut entries).await {
        Ok(count) => {
            for i in 0..count {
                karythra::io::print(entries[i].name_str());
                karythra::io::print("  ");
                karythra::io::print_dec(entries[i].size as u64);
                karythra::io::print("\n");
            }
        }
        Err(_) => karythra::io::print("ls: fs error\n"),
    }
}
