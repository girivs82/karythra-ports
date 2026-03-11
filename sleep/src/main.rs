#![no_std]
#![no_main]

fn parse_u64(s: &str) -> Option<u64> {
    if s.is_empty() { return None; }
    let mut val: u64 = 0;
    for &b in s.as_bytes() {
        if b < b'0' || b > b'9' { return None; }
        val = val.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(val)
}

#[karythra::main]
async fn main(args: &str) {
    let s = args.trim_ascii();
    match parse_u64(s) {
        Some(ms) => {
            karythra::time::sleep(ms).await;
            karythra::io::print("slept ");
            karythra::io::print_dec(ms);
            karythra::io::print("ms\n");
        }
        None => karythra::io::print("usage: sleep <milliseconds>\n"),
    }
}
