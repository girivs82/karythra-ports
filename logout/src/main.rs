#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    let _ = karythra::fs::session_join().await;
    match karythra::fs::session_logout().await {
        Ok(()) => karythra::io::print("logged out\n"),
        Err(_) => karythra::io::print("not in a session\n"),
    }
}
