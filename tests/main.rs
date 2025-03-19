// -- Imports and setup ---
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use ftl::ftl::FTL;
use ftl::media_manager::operations::MediaOperations;

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

// #[test_case]
// pub fn ftl() {
//     let mm: MockMediaManager = MockMediaManager::new();
//     let global_ftl: FTL<MockMediaManager> = FTL::new(mm);
// }

// use tests::ftl;

mod unit;