use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
};

use core::mem::transmute_copy;

use super::operations::{MediaManagerError, MediaManager};

pub struct MediaManagerStub;

impl MediaManagerStub {
    pub const fn new() -> Self {
        MediaManagerStub {}
    }
}

impl MediaManager for MediaManagerStub {
    fn erase_block(_pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    fn read_page<T>(_ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn read_block<T>(_pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(_ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}
