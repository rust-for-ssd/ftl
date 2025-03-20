use crate::config::{
    BLOCKS_PER_PLANE, LUNS_PER_CHANNEL, N_CHANNELS, PAGES_PER_BLOCK, PLANES_PER_LUN, TOTAL_PAGES,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalPageAddress {
    pub channel_id: usize,
    pub lun_id: usize,
    pub plane_id: usize,
    pub block_id: usize,
    pub page_id: usize,
}

pub struct PhysicalBlockAddress {
    pub channel_id: usize,
    pub lun_id: usize,
    pub plane_id: usize,
    pub block_id: usize,
}

impl PhysicalPageAddress {
    pub fn is_reserved(&self) -> bool {
        // We reserve block 0 for bbt metadata by choice
        return self.block_id == 0;
    }
}

pub type CompactPhysicalPageAddress = usize;
pub type LogicalPageAddress = usize; //range [0, config::N_pages]

impl From<CompactPhysicalPageAddress> for PhysicalPageAddress {
    fn from(cppa: CompactPhysicalPageAddress) -> Self {
        let channel = cppa / (TOTAL_PAGES / N_CHANNELS);
        let mut remainder = cppa % (TOTAL_PAGES / N_CHANNELS);
        let lun = remainder / (PLANES_PER_LUN * BLOCKS_PER_PLANE * PAGES_PER_BLOCK);
        remainder = remainder % (PLANES_PER_LUN * BLOCKS_PER_PLANE * PAGES_PER_BLOCK);
        let plane = remainder / (BLOCKS_PER_PLANE * PAGES_PER_BLOCK);
        remainder = remainder % (BLOCKS_PER_PLANE * PAGES_PER_BLOCK);
        let block = remainder / PAGES_PER_BLOCK;
        let page = remainder % PAGES_PER_BLOCK;

        debug_assert!(channel < N_CHANNELS);
        debug_assert!(lun < LUNS_PER_CHANNEL);
        debug_assert!(plane < PLANES_PER_LUN);
        debug_assert!(block < BLOCKS_PER_PLANE);
        debug_assert!(page < PAGES_PER_BLOCK);

        PhysicalPageAddress {
            channel_id: channel,
            lun_id: lun,
            plane_id: plane,
            block_id: block,
            page_id: page,
        }
    }
}

impl Into<usize> for PhysicalPageAddress {
    fn into(self) -> CompactPhysicalPageAddress {
        debug_assert!(self.channel_id < N_CHANNELS);
        debug_assert!(self.lun_id < LUNS_PER_CHANNEL);
        debug_assert!(self.plane_id < PLANES_PER_LUN);
        debug_assert!(self.block_id < BLOCKS_PER_PLANE);
        debug_assert!(self.page_id < PAGES_PER_BLOCK);

        let channel_offset = self.channel_id * (TOTAL_PAGES / N_CHANNELS);
        let lun_offset = self.lun_id * PLANES_PER_LUN * BLOCKS_PER_PLANE * PAGES_PER_BLOCK;
        let plane_offset = self.plane_id * BLOCKS_PER_PLANE * PAGES_PER_BLOCK;
        let block_offset = self.block_id * PAGES_PER_BLOCK;
        channel_offset + lun_offset + plane_offset + block_offset + self.page_id
    }
}

impl PhysicalBlockAddress {
    pub fn is_reserved(&self) -> bool {
        // We reserve block 0 for bbt metadata by choice
        return self.block_id == 0;
    }
}
