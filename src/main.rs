#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {};
}

#[no_mangle]
fn kernel_init() -> ! {
    loop {};
}
