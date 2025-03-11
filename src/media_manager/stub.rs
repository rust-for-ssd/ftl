pub struct PhysicalPageAddress {
    channel: usize,
    lun: usize,
    plane: u8,
    block: usize,
    page: usize,
}

pub struct PhysicalBlockAddress {
    pub channel: usize,
    pub lun: usize,
    pub plane: u8,
    pub block: usize,
}

pub struct MediaManger {}

type C_ERR = usize;

impl MediaManger {
    pub fn erase_block(pba: &PhysicalBlockAddress) -> Result<(), C_ERR> {
        todo!()
    }

    pub fn read_page(ppa: &PhysicalPageAddress) -> Result<(), C_ERR> {
        // TODO: should have a proper return type instead of Ok(())
        todo!()
    }

    pub fn write_page(ppa: &PhysicalPageAddress) -> Result<(), C_ERR> {
        todo!()
    }
}
