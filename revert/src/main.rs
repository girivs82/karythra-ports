#![no_std]
#![no_main]

fn split_first(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], s[i + 1..].trim_ascii()),
        None => (s, ""),
    }
}

fn parse_u16(s: &str) -> Option<u16> {
    let mut val: u16 = 0;
    for b in s.bytes() {
        if b < b'0' || b > b'9' { return None; }
        val = val.checked_mul(10)?.checked_add((b - b'0') as u16)?;
    }
    Some(val)
}

#[karythra::main]
async fn main(args: &str) {
    let (name, ver_str) = split_first(args.trim_ascii());
    if name.is_empty() || ver_str.is_empty() {
        karythra::io::print("usage: revert <file> <version>\n");
        return;
    }
    let version = match parse_u16(ver_str) {
        Some(v) => v,
        None => {
            karythra::io::print("revert: invalid version number\n");
            return;
        }
    };
    match karythra::fs::revert(name, version).await {
        Ok(()) => {
            karythra::io::print("reverted ");
            karythra::io::print(name);
            karythra::io::print(" to version ");
            karythra::io::print_dec(version as u64);
            karythra::io::print("\n");
        }
        Err(_) => karythra::io::print("revert: failed\n"),
    }
}
