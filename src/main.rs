#![no_std] // Do not use standard library
#![no_main] // Do not use the normal entry point chain. We will overwrite crt0 entry point directly.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Disable name mangling
             // Also, use the C calling convention
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
