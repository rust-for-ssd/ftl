// Page provison: gives a physical page adress (ppa) to an available page

use crate::media_manager::stub::{
    C_ERR, MEDIA_MANAGER, MediaManger, PhysicalBlockAddress, PhysicalPageAddress,
};


pub struct GlobalProvisoner {
    channel_provisioners: [ChannelProvisioner; MEDIA_MANAGER.n_channels],
}

impl GlobalProvisoner {
    pub fn provision_block() {todo!()}
    pub fn provison_page() {todo!()}
}

struct ChannelProvisioner {
    luns: [LUN; MEDIA_MANAGER.n_luns],
    n_luns: usize,
}

#[derive(Copy, Clone)]
struct Channel {
    luns: [LUN; MEDIA_MANAGER.n_luns],
    n_luns: usize,
}

#[derive(Copy, Clone)]
struct LUN { // TODO: we make these way too big for starters
    free: [Block; MEDIA_MANAGER.n_blocks * MEDIA_MANAGER.n_planes],
    used: [Block; MEDIA_MANAGER.n_blocks * MEDIA_MANAGER.n_planes],
    partially_used: [BlockWithPageInfo; MEDIA_MANAGER.n_blocks * MEDIA_MANAGER.n_planes],
}

#[derive(Copy, Clone)]
struct Block {
    id: usize,
}

#[derive(Copy, Clone)]
struct BlockWithPageInfo {
    id: usize,
    page: [Page; MEDIA_MANAGER.n_pages],
    n_pages: usize,
}

#[derive(Copy, Clone)]
struct Page {
    dirty: bool,
}


impl ChannelProvisioner {
    fn provision_block() -> Result<PhysicalBlockAddress, C_ERR> {
        
    }
    fn provison_page() -> Result<PhysicalPageAddress, C_ERR> {todo!()}

}



// To provision a block we need: 
// - It's free 
// - It's not bad (not in bb table)

// - Extra: 
// - We don't want to provison two blocks in the same lun, since we cannot parallelize I/Os then.
// - Maybe round-robin fashino 