#![no_std]
#![no_main]

#[karythra::main]
async fn main() {
    let ms = karythra::time::uptime_ms();
    let secs = ms / 1000;
    let mins = secs / 60;
    karythra::io::print("up ");
    karythra::io::print_dec(mins);
    karythra::io::print("m ");
    karythra::io::print_dec(secs % 60);
    karythra::io::print("s\n");
}
