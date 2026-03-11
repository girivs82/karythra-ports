#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    let name = args.trim_ascii();
    if name.is_empty() {
        karythra::io::print("usage: rm <filename>\n");
        return;
    }
    match karythra::fs::delete(name).await {
        Ok(()) => {
            karythra::io::print("removed ");
            karythra::io::print(name);
            karythra::io::print("\n");
        }
        Err(_) => {
            karythra::io::print("rm: failed to remove ");
            karythra::io::print(name);
            karythra::io::print("\n");
        }
    }
}
