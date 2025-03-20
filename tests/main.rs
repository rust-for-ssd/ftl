// -- Imports and setup ---
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use ftl::config::{TOTAL_GB};
use ftl::unsafeprintln;
use ftl::utils::print::QemuUart;
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
    unsafeprintln!("--------------------------");
    unsafeprintln!("Testing SSD of size {} GB", TOTAL_GB);
    unsafeprintln!("--------------------------");

    test_main();
    loop {}
}

mod unit;
