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
    let (cmd, rest) = split_first(args.trim_ascii());
    match cmd {
        "add" => {
            let (file, tag) = split_first(rest);
            if file.is_empty() || tag.is_empty() {
                karythra::io::print("usage: tag add <file> <tag>\n");
                return;
            }
            match karythra::fs::tag(file, tag).await {
                Ok(()) => {
                    karythra::io::print("tagged ");
                    karythra::io::print(file);
                    karythra::io::print(" with ");
                    karythra::io::print(tag);
                    karythra::io::print("\n");
                }
                Err(_) => karythra::io::print("tag: failed\n"),
            }
        }
        "rm" => {
            let (file, tag) = split_first(rest);
            if file.is_empty() || tag.is_empty() {
                karythra::io::print("usage: tag rm <file> <tag>\n");
                return;
            }
            match karythra::fs::untag(file, tag).await {
                Ok(()) => {
                    karythra::io::print("untagged ");
                    karythra::io::print(tag);
                    karythra::io::print(" from ");
                    karythra::io::print(file);
                    karythra::io::print("\n");
                }
                Err(_) => karythra::io::print("tag: failed\n"),
            }
        }
        "ls" => {
            let file = rest.trim_ascii();
            if file.is_empty() {
                karythra::io::print("usage: tag ls <file>\n");
                return;
            }
            match karythra::fs::tags(file).await {
                Ok(reply) => {
                    if reply.msg_type == 100 {
                        let n = reply.len as usize;
                        if let Ok(s) = core::str::from_utf8(&reply.data[..n]) {
                            karythra::io::print(s);
                        }
                    } else {
                        karythra::io::print("no tags\n");
                    }
                }
                Err(_) => karythra::io::print("tag: fs error\n"),
            }
        }
        _ => karythra::io::print("usage: tag <add|rm|ls> <file> [tag]\n"),
    }
}
