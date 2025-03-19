// -- Imports and setup ---
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use riscv_rt::entry;

// -- Custom panic handler
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    rv_unit::test_panic_handler(info);
    test_main();
    loop {}
}

#[entry]
fn main() -> ! {
    test_main();
    loop {}
}

mod utils;
