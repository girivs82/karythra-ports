#![no_std]
#![no_main]

fn split_first(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], s[i + 1..].trim_ascii()),
        None => (s, ""),
    }
}

fn label_str(label: &[u8; 16]) -> &str {
    let len = label.iter().position(|&b| b == 0).unwrap_or(16);
    core::str::from_utf8(&label[..len]).unwrap_or("?")
}

#[karythra::main]
async fn main(args: &str) {
    let _ = karythra::fs::session_join().await;
    let (sub, rest) = split_first(args.trim_ascii());
    match sub {
        "create" => {
            if rest.is_empty() {
                karythra::io::print("usage: ns create <label>\n");
                return;
            }
            match karythra::fs::ns_create(rest).await {
                Ok(idx) => {
                    karythra::io::print("namespace ");
                    karythra::io::print(rest);
                    karythra::io::print(" created (");
                    let mut buf = [0u8; 4];
                    let n = karythra::fmt::decimal(idx as u64, &mut buf);
                    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
                        karythra::io::print(s);
                    }
                    karythra::io::print(")\n");
                }
                Err(_) => karythra::io::print("ns create failed (not logged in or limit reached)\n"),
            }
        }
        "switch" => {
            if rest.is_empty() {
                karythra::io::print("usage: ns switch <label>\n");
                return;
            }
            match karythra::fs::ns_switch(rest).await {
                Ok(()) => {
                    karythra::io::print("switched to ");
                    karythra::io::print(rest);
                    karythra::io::print("\n");
                }
                Err(_) => karythra::io::print("ns switch failed\n"),
            }
        }
        "" | "list" => {
            match karythra::fs::ns_list().await {
                Ok(list) => {
                    if list.count == 0 {
                        karythra::io::print("no namespaces\n");
                        return;
                    }
                    for i in 0..list.count as usize {
                        if i == list.active as usize {
                            karythra::io::print("* ");
                        } else {
                            karythra::io::print("  ");
                        }
                        karythra::io::print(label_str(&list.labels[i]));
                        karythra::io::print("\n");
                    }
                }
                Err(_) => karythra::io::print("not in a session\n"),
            }
        }
        _ => karythra::io::print("usage: ns [list|create <label>|switch <label>]\n"),
    }
}
