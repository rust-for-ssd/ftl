pub struct PhysicalPageAddress {
    pub channel: usize,
    pub lun: usize,
    pub plane: usize,
    pub block: usize,
    pub page: usize,
}

pub struct PhysicalBlockAddress {
    pub channel: usize,
    pub lun: usize,
    pub plane: usize,
    pub block: usize,
}
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

impl PhysicalPageAddress {
    pub fn is_reserved(&self) -> bool {
        todo!()
    }
}

impl PhysicalBlockAddress {
    pub fn is_reserved(&self) -> bool {
        todo!()
    }
}
