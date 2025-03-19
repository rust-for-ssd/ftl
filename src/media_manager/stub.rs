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
    pub const fn new() -> Self {
        MediaManager {}
    }
}

pub trait MediaOperations {
    fn erase_block(&self, pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError>;

    fn read_page<T>(&self, ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError>;

    fn read_block<T>(&self, pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError>;

    fn write_page(&self, ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError>;
}

impl MediaOperations for MediaManager {
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

pub struct MockMediaManager {}

impl MockMediaManager {
    pub const fn new() -> Self { MockMediaManager{}}
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
