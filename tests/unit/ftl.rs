use core::mem::{size_of, transmute_copy};
use ftl::core::address::LogicalPageAddress;
use ftl::utils::print::QemuUart;
use ftl::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::FTL,
    media_manager::operations::{MediaManagerError, MediaOperations},
    unsafeprintln,
};

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

    fn read_page<T>(&self, _ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn read_block<T>(&self, _pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(&self, _ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}

#[test_case]
pub fn init_ftl() {
    // let mm: MockMediaManager = MockMediaManager::new();
    // let _global_ftl: FTL<MockMediaManager> = FTL::new(mm);

    let ftl_size = size_of::<FTL>();
    // let global_ftl_size = size_of
    let mm_size = size_of::<MockMediaManager>();

    // Override the global MEDIA_MANAGER for testing
    // pub static MEDIA_MANAGER: MockMediaManager = MockMediaManager::new();
    let mut ftl: FTL = FTL::new();
    let res = ftl.write_page(100);
    match res {
        Ok(_) => {
            unsafeprintln!("Page written successfully");
        },
        Err(e) => {
            unsafeprintln!("Failed to write page");
        }
    }

    let content = ftl.read_page(100);
    match content {
        Ok(data) => {
            // Use the data
            unsafeprintln!("Page read successfully");
        },
        Err(e) => {
            unsafeprintln!("Failed to read page");
        }
    }
    // let result: Result<[u8; config::BYTES_PER_BLOCK], MediaManagerError> = 
    // MEDIA_MANAGER.read_block(&PhysicalBlockAddress { 
    //     channel: 0, 
    //     lun: 0, 
    //     plane: 0, 
    //     block: 0 
    // });

// Handle the result
// match result {
//     Ok(data) => {
//         // Use the data
//         unsafeprintln!("Block read successfully");
//     },
//     Err(e) => {
//         unsafeprintln!("Failed to read block");
//     }
// }
    

    unsafeprintln!("FTL size is {} MB", ftl_size as f32 / (1024.0 * 1024.0));
    unsafeprintln!("MM size is {} MB", mm_size as f32 / (1024.0 * 1024.0));
}
