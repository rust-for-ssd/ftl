use core::mem::{size_of, transmute_copy};
use ftl::utils::print::QemuUart;
use ftl::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::FTL,
    media_manager::operations::{MediaManagerError, MediaOperations},
    unsafeprintln,
};
use semihosting::println;

pub struct MockMediaManager {}

impl MediaOperations for MockMediaManager {
    fn erase_block(_pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    fn read_page<T>(_ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        println!("I AM A MOCK");
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn read_block<T>(_pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(_ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        println!("I AM A MOCK");
        Ok(())
    }
}

#[test_case]
pub fn ftl() {
    let mut global_ftl: FTL<MockMediaManager> = FTL::new();
    let content = global_ftl.write_page(100);
    match content {
        Err(ftl::ftl::FtlErr::WritePage(s)) => println!("{}", s),
        Err(_) => println!("ERR"),
        Ok(_) => println!("OK"),
    }
}
