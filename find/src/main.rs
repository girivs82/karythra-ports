#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let input = args.trim_ascii();
    if input.is_empty() {
        karythra::io::print("usage: find <tag> [tag ...]\n");
        return;
    }
    let mut tags: [&str; 8] = [""; 8];
    let mut count = 0;
    let mut remaining = input;
    while !remaining.is_empty() && count < 8 {
        match remaining.find(' ') {
            Some(i) => {
                let tag = remaining[..i].trim_ascii();
                if !tag.is_empty() {
                    tags[count] = tag;
                    count += 1;
                }
                remaining = remaining[i + 1..].trim_ascii();
            }
            None => {
                tags[count] = remaining;
                count += 1;
                remaining = "";
            }
        }
    }
    match karythra::fs::find(&tags[..count]).await {
        Ok(reply) => {
            if reply.msg_type == 100 {
                let n = reply.len as usize;
                if let Ok(s) = core::str::from_utf8(&reply.data[..n]) {
                    karythra::io::print(s);
                }
            } else {
                karythra::io::print("no matches\n");
            }
        }
        Err(_) => karythra::io::print("find: fs error\n"),
    }
}
