#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let _ = karythra::fs::session_join().await;
    let name = args.trim_ascii();
    if name.is_empty() {
        karythra::io::print("usage: share <file>\n");
        return;
    }
    match karythra::fs::share(name).await {
        Ok(()) => {
            karythra::io::print("shared ");
            karythra::io::print(name);
            karythra::io::print(" to system\n");
        }
        Err(_) => karythra::io::print("share failed (not in session or file not found)\n"),
    }
}
