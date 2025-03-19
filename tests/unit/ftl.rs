use core::mem::transmute_copy;
use ftl::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::FTL,
    media_manager::operations::{MediaManagerError, MediaOperations},
};

pub struct MockMediaManager {}

impl MockMediaManager {
    pub const fn new() -> Self {
        MockMediaManager {}
    }
}

impl MediaOperations for MockMediaManager {
    fn erase_block(&self, pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    fn read_page<T>(&self, ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn read_block<T>(&self, pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(&self, ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}

#[test_case]
pub fn ftl() {
    let mm: MockMediaManager = MockMediaManager::new();
    let global_ftl: FTL<MockMediaManager> = FTL::new(mm);
}
