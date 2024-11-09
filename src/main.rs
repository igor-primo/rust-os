#![no_std] // Do not use standard library
#![no_main] // Do not use the normal entry point chain. We will overwrite crt0 entry point directly.

use core::panic::PanicInfo;

#[no_mangle] // Disable name mangling
             // Also, use the C calling convention
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
