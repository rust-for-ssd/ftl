use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
};

use core::mem::transmute_copy;

pub struct MediaManager;
pub enum PhysicalBlockAddressError {
    Reserved,
    InvalidAddress,
    BadBlock,
}

pub enum MediaManagerError {
    Write,
    Read,
    Erase,
}

impl MediaManager {
    pub fn erase_block(pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    pub fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    pub fn write_page(ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}
