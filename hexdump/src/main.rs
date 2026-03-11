#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let name = args.trim_ascii();
    if name.is_empty() {
        karythra::io::print("usage: hexdump <filename>\n");
        return;
    }
    let mut buf = [0u8; 2048];
    match karythra::fs::read_into(name, &mut buf).await {
        Ok(n) => {
            let mut line = [0u8; 80];
            let mut offset = 0;
            while offset < n {
                let mut pos = 0;
                let addr_len = karythra::fmt::hex(offset as u64, &mut line[pos..pos + 8]);
                pos += addr_len;
                while pos < 8 { line[pos] = b' '; pos += 1; }
                line[pos] = b' '; pos += 1;
                line[pos] = b' '; pos += 1;
                let row_end = (offset + 16).min(n);
                for i in offset..offset + 16 {
                    if i < row_end {
                        let hi = buf[i] >> 4;
                        let lo = buf[i] & 0x0f;
                        line[pos] = if hi < 10 { b'0' + hi } else { b'a' + hi - 10 };
                        line[pos + 1] = if lo < 10 { b'0' + lo } else { b'a' + lo - 10 };
                    } else {
                        line[pos] = b' ';
                        line[pos + 1] = b' ';
                    }
                    pos += 2;
                    line[pos] = b' '; pos += 1;
                    if i == offset + 7 { line[pos] = b' '; pos += 1; }
                }
                line[pos] = b'|'; pos += 1;
                for i in offset..row_end {
                    let b = buf[i];
                    line[pos] = if b >= 0x20 && b < 0x7f { b } else { b'.' };
                    pos += 1;
                }
                line[pos] = b'|'; pos += 1;
                line[pos] = b'\n'; pos += 1;
                if let Ok(s) = core::str::from_utf8(&line[..pos]) {
                    karythra::io::print(s);
                }
                offset += 16;
            }
        }
        Err(_) => {
            karythra::io::print("not found: ");
            karythra::io::print(name);
            karythra::io::print("\n");
        }
    }
}
