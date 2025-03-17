use crate::core::address::{PhysicalBlockAddress, PhysicalPageAddress};

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
        todo!()
    }

    pub fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // TODO: should have a proper return type instead of Ok(())
        todo!()
    }

    pub fn write_page(ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        todo!()
    }
}
