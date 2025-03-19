use core::sync;

use crate::core::address::{PhysicalBlockAddress, PhysicalPageAddress};

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

pub trait MediaOperations: Sync {
    fn erase_block(&self, pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError>;

    // fn read_page<T>(&self, ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError>;

    // fn read_block<T>(&self, pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError>;

    fn write_page(&self, ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError>;
}
