use core::mem::{size_of, transmute_copy};
use ftl::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::FTL,
    media_manager::operations::{MediaManagerError, MediaOperations},
};
use semihosting::println;

pub struct MockMediaManager {}

impl MediaOperations for MockMediaManager {
    fn erase_block(_pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate

        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn read_block<T>(_pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}

#[test_case]
pub fn ftl() {
    let mut ftl: FTL<MockMediaManager> = FTL::new();
    ftl.init();

    for lpa in 0..40 {
        let _ = ftl.write_page(lpa);
    }
}
