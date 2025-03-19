use crate::core::address::{PhysicalBlockAddress, PhysicalPageAddress};

pub trait MediaManager {
    fn erase_block(pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError>;

    fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError>;

    fn write_page(ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError>;
}

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
