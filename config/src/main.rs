#![no_std]
#![no_main]

use karythra::io::{print, print_dec};

fn name_str(buf: &[u8; 16]) -> &str {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(16);
    core::str::from_utf8(&buf[..len]).unwrap_or("?")
}

fn type_name(t: u8) -> &'static str {
    match t {
        0 => "u8",
        1 => "u16",
        2 => "u32",
        3 => "bool",
        _ => "?",
    }
}

fn read_field(data: &[u8], offset: u8, size: u8) -> u32 {
    let off = offset as usize;
    match size {
        1 => { if off < data.len() { data[off] as u32 } else { 0 } }
        2 => {
            if off + 2 <= data.len() {
                u16::from_le_bytes([data[off], data[off + 1]]) as u32
            } else { 0 }
        }
        4 => {
            if off + 4 <= data.len() {
                u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            } else { 0 }
        }
        _ => 0,
    }
}

fn write_field(data: &mut [u8], offset: u8, size: u8, value: u32) {
    let off = offset as usize;
    match size {
        1 => { if off < data.len() { data[off] = value as u8; } }
        2 => {
            if off + 2 <= data.len() {
                data[off..off + 2].copy_from_slice(&(value as u16).to_le_bytes());
            }
        }
        4 => {
            if off + 4 <= data.len() {
                data[off..off + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
        _ => {}
    }
}

fn parse_u32(s: &str) -> Option<u32> {
    let mut val: u32 = 0;
    if s.is_empty() { return None; }
    for &b in s.as_bytes() {
        if b < b'0' || b > b'9' { return None; }
        val = val.checked_mul(10)?.checked_add((b - b'0') as u32)?;
    }
    Some(val)
}

async fn cmd_list() {
    match karythra::fs::find(&["config"]).await {
        Ok(reply) => {
            if reply.msg_type == 100 {
                let n = reply.len as usize;
                if let Ok(s) = core::str::from_utf8(&reply.data[..n]) {
                    print(s);
                }
            } else {
                print("(no config blobs)\n");
            }
        }
        Err(_) => print("config: fs error\n"),
    }
}

async fn cmd_show(service: &str) {
    let desc = match karythra::config::describe(service).await {
        Ok(d) => d,
        Err(_) => { print("config: service not found: "); print(service); print("\n"); return; }
    };
    let data = match karythra::config::get_raw(service).await {
        Ok(d) => d,
        Err(_) => { print("config: get failed\n"); return; }
    };

    print(name_str(&desc.service_name));
    print(" (");
    print_dec(desc.struct_size as u64);
    print(" bytes, ");
    print_dec(desc.field_count as u64);
    print(" fields)\n");

    let fc = (desc.field_count as usize).min(16);
    for i in 0..fc {
        let f = &desc.fields[i];
        let n = name_str(&f.name);
        if n.starts_with('_') { continue; }
        let val = read_field(&data.data[..data.len], f.offset, f.size);
        print("  ");
        print(n);
        print(": ");
        print_dec(val as u64);
        if val == f.default_value {
            print(" (default)");
        }
        print("  [");
        print(type_name(f.field_type));
        print("]\n");
    }
}

async fn cmd_set(service: &str, field_name: &str, value_str: &str) {
    let value = match parse_u32(value_str) {
        Some(v) => v,
        None => { print("config: invalid value\n"); return; }
    };

    let desc = match karythra::config::describe(service).await {
        Ok(d) => d,
        Err(_) => { print("config: service not found\n"); return; }
    };

    let fc = (desc.field_count as usize).min(16);
    let mut found = false;
    let mut offset = 0u8;
    let mut size = 0u8;
    for i in 0..fc {
        if name_str(&desc.fields[i].name) == field_name {
            found = true;
            offset = desc.fields[i].offset;
            size = desc.fields[i].size;
            break;
        }
    }
    if !found {
        print("config: unknown field: ");
        print(field_name);
        print("\n");
        return;
    }

    let mut data = match karythra::config::get_raw(service).await {
        Ok(d) => d,
        Err(_) => { print("config: get failed\n"); return; }
    };

    write_field(&mut data.data[..data.len], offset, size, value);

    match karythra::config::set_raw(service, &data.data[..data.len]).await {
        Ok(()) => {
            print(field_name);
            print(" = ");
            print_dec(value as u64);
            print("\n");
        }
        Err(_) => print("config: set failed\n"),
    }
}

async fn cmd_reset(service: &str) {
    let config_name_prefix = "config/";
    if service.len() + config_name_prefix.len() <= 47 {
        let mut name_buf = [0u8; 48];
        name_buf[..config_name_prefix.len()].copy_from_slice(config_name_prefix.as_bytes());
        let slen = service.len().min(47 - config_name_prefix.len());
        name_buf[config_name_prefix.len()..config_name_prefix.len() + slen]
            .copy_from_slice(&service.as_bytes()[..slen]);
        let full_name = core::str::from_utf8(&name_buf[..config_name_prefix.len() + slen])
            .unwrap_or("");
        let _ = karythra::fs::delete(full_name).await;
    }

    match karythra::config::reset(service).await {
        Ok(()) => print("config reset to defaults\n"),
        Err(_) => print("config: reset failed\n"),
    }
}

#[karythra::main]
async fn main(args: &str) {
    let input = args.trim_ascii();
    if input.is_empty() || input == "help" {
        print("usage: config <list|show|set|reset> ...\n");
        print("  config list\n");
        print("  config show <service>\n");
        print("  config set <service> <field> <value>\n");
        print("  config reset <service>\n");
        return;
    }

    let (cmd, rest) = match input.find(' ') {
        Some(i) => (&input[..i], input[i + 1..].trim_ascii()),
        None => (input, ""),
    };

    match cmd {
        "list" => cmd_list().await,
        "show" => {
            if rest.is_empty() {
                print("usage: config show <service>\n");
            } else {
                cmd_show(rest).await;
            }
        }
        "set" => {
            let parts: [&str; 3] = split3(rest);
            if parts[0].is_empty() || parts[1].is_empty() || parts[2].is_empty() {
                print("usage: config set <service> <field> <value>\n");
            } else {
                cmd_set(parts[0], parts[1], parts[2]).await;
            }
        }
        "reset" => {
            if rest.is_empty() {
                print("usage: config reset <service>\n");
            } else {
                cmd_reset(rest).await;
            }
        }
        _ => {
            print("config: unknown command: ");
            print(cmd);
            print("\n");
        }
    }
}

fn split3(s: &str) -> [&str; 3] {
    let mut result = [""; 3];
    let mut remaining = s;
    for i in 0..3 {
        let trimmed = remaining.trim_ascii();
        if trimmed.is_empty() { break; }
        if i == 2 {
            result[i] = trimmed;
            break;
        }
        match trimmed.find(' ') {
            Some(pos) => {
                result[i] = &trimmed[..pos];
                remaining = &trimmed[pos + 1..];
            }
            None => {
                result[i] = trimmed;
                break;
            }
        }
    }
    result
}
