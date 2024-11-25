#![no_std] // Do not use standard library
#![no_main] // Do not use the normal entry point chain. We will overwrite crt0 entry point directly.
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;
//use rust_os::println;

#[no_mangle] // Disable name mangling
             // Also, use the C calling convention
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}
