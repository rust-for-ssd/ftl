use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
};

use core::mem::transmute_copy;

use super::operations::{MediaManagerError, MediaOperations};

pub struct MediaManager;

impl MediaManager {
    pub const fn new() -> Self {
        MediaManager {}
    }
}

impl MediaOperations for MediaManager {
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
