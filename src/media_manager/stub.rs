pub struct PhysicalPageAddress {
    pub channel: usize,
    pub lun: usize,
    pub plane: u8,
    pub block: usize,
    pub page: usize,
}

pub struct PhysicalBlockAddress {
    pub channel: usize,
    pub lun: usize,
    pub plane: u8,
    pub block: usize,
}

// NOTE: MediaManager should contain Safe C-wrappers around whatever MM strub we get from Ivan
pub struct MediaManger {
    pub n_luns: usize,
    pub n_planes: usize,
    pub n_blocks: usize,
    pub n_pages: usize,
}

pub type C_ERR = usize;

pub static MEDIA_MANAGER: MediaManger = MediaManger {
    n_luns: 32,
    n_planes: 2,
    n_blocks: 64,
    n_pages: 8,
};

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
