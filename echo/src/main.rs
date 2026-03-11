#![no_std]
#![no_main]

#[karythra::main]
async fn main(args: &str) {
    karythra::io::print(args);
    karythra::io::print("\n");
}
