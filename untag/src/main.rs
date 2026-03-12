#![no_std]
#![no_main]

fn split_first(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], s[i + 1..].trim_ascii()),
        None => (s, ""),
    }
}

#[karythra::main]
async fn main(args: &str) {
    let (name, tag) = split_first(args.trim_ascii());
    if name.is_empty() || tag.is_empty() {
        karythra::io::print("usage: untag <file> <tag>\n");
        return;
    }
    match karythra::fs::untag(name, tag).await {
        Ok(()) => {
            karythra::io::print("untagged ");
            karythra::io::print(tag);
            karythra::io::print(" from ");
            karythra::io::print(name);
            karythra::io::print("\n");
        }
        Err(_) => karythra::io::print("untag: failed\n"),
    }
}
