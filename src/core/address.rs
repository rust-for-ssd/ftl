use crate::config::{BLOCKS_PER_PLANE, N_CHANNELS, PAGES_PER_BLOCK, PLANES_PER_LUN, TOTAL_PAGES};


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


impl PhysicalPageAddress {
    pub fn is_reserved(&self) -> bool {
        todo!()
    }
}

pub type CompactPhysicalPageAddress = usize; 
pub type LogicalPageAddress = usize; //range [0, config::N_pages]


impl Into<usize> for PhysicalPageAddress {
    fn into(self) -> CompactPhysicalPageAddress {
        let channel_offset = self.channel * (TOTAL_PAGES / N_CHANNELS);
        let lun_offset = self.lun * PLANES_PER_LUN * BLOCKS_PER_PLANE * PAGES_PER_BLOCK;
        let plane_offset = self.plane * BLOCKS_PER_PLANE * PAGES_PER_BLOCK;
        let block_offset = self.block * PAGES_PER_BLOCK;
        channel_offset + lun_offset + plane_offset + block_offset + self.page
    }
}

impl PhysicalBlockAddress {
    pub fn is_reserved(&self) -> bool {
        todo!()
    }
}