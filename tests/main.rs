// -- Imports and setup ---
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use ftl::config::{TOTAL_GB, TOTAL_MB};
use riscv_rt::entry;
use rv_unit::println_red;
use semihosting::println;

// -- Custom panic handler
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    rv_unit::test_panic_handler(info);
    test_main();
    loop {}
}

#[entry]
fn main() -> ! {
    #[cfg(not(feature = "qemu"))]
    {
        println_red!("--------------------------");
        println_red!("WARNING!: NOT RUNNING WITH TEST SIZES FOR QEMU");
        println_red!("Use: cargo t --features qemu");
        println_red!("--------------------------");
    }

    println!("--------------------------");
    println!("Testing SSD of size {} GB", TOTAL_GB);
    println!("--------------------------");

    test_main();
    loop {}
}

mod unit;
