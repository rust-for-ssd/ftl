use core::mem::{size_of, transmute_copy};
use ftl::ftl::GLOBAL_FTL;
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

impl MockMediaManager {
    pub const fn new() -> Self {
        MockMediaManager {}
    }
}

impl MediaOperations for MockMediaManager {
    fn erase_block(&self, _pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    // fn read_page<T>(&self, _ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
    //     // We simulate
    //     let page = [0; config::BYTES_PER_PAGE];
    //     Ok(unsafe { transmute_copy::<_, T>(&page) })
    // }

    // fn read_block<T>(&self, _pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
    //     todo!()
    // }

    fn write_page(&self, _ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        println!("I AM A NEW FTL");
        Ok(())
    }
}
static MM: MockMediaManager = MockMediaManager::new();

#[test_case]
pub fn ftl() {
    unsafe {
        GLOBAL_FTL = FTL::new(&MM);
        let mm = GLOBAL_FTL.mm;
        let _ = mm.write_page(&PhysicalPageAddress {
            channel: 0,
            lun: 0,
            plane: 0,
            block: 0,
            page: 0,
        });
    }
    // GLOBAL_FTL = _global_ftl;

    let ftl_size = size_of::<FTL>();
    // let global_ftl_size = size_of
    unsafeprintln!("FTL size is {} MB", ftl_size as f32 / (1024.0 * 1024.0));
}
