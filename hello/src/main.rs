#![no_std]
#![no_main]

use karythra::io;

#[no_mangle]
pub extern "C" fn _start(state: *mut u8) -> u8 {
    let kapi_ptr = unsafe { *(state as *const u64) } as *const karythra::KernelApi;
    karythra::init(kapi_ptr);

    io::print("Hello from Karythra!\n");
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
