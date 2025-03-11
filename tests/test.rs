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
    #[cfg(test)]
    test_main();
    loop {}
}

use ssd_rs::add;
// --- Example: basic test cases ---

#[test_case]
pub fn test_basic_positive() {
    let x = add(1, 2);
    assert_eq!(x, 3);
    assert_eq!(1, 1);
    assert_eq!(42, 42);
    assert!(true);
}

#[test_case]
pub fn test_basic_negative() {
    assert_ne!(1, 2);
    assert_ne!(42, 0);
    assert!(!false);
}

#[test_case]
pub fn test_basic_zero() {
    assert_eq!(0, 0);
    assert_ne!(0, 1);
}

#[test_case]
pub fn test_negative() {
    assert_eq!(1, 2);
}
