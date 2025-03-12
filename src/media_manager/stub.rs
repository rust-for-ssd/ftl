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
pub struct MediaManager {
    pub n_channels: usize,
    pub n_luns: usize,
    pub n_planes: usize,
    pub n_blocks_per_plane: usize,
    pub n_pages: usize,
    pub n_blocks_per_lun: usize,
}

pub type C_ERR = usize;

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

const N_CHANNELS: usize = 24;
const N_LUNS: usize = 32;
const N_PLANES: usize = 2;
const N_BLOCKS_PER_PLANE: usize = 64;
const N_PAGES: usize = 8;
pub const N_BLOCKS_PER_LUN: usize = N_PLANES * N_BLOCKS_PER_PLANE;

pub static MEDIA_MANAGER: MediaManager = MediaManager {
    n_channels: N_CHANNELS,
    n_luns: N_LUNS,
    n_planes: N_PLANES,
    n_blocks_per_plane: N_BLOCKS_PER_PLANE,
    n_pages: N_PAGES,
    n_blocks_per_lun: N_PLANES * N_BLOCKS_PER_PLANE,
};

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
