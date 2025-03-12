// Page provison: gives a physical page adress (ppa) to an available page

use crate::{
    media_manager::stub::{
        C_ERR, MEDIA_MANAGER, N_BLOCKS_PER_LUN, PhysicalBlockAddress, PhysicalPageAddress,
    },
    utils::ring_buffer::RingBuffer,
};

pub struct GlobalProvisoner {
    channel_provisioners: [ChannelProvisioner; MEDIA_MANAGER.n_channels],
}

impl GlobalProvisoner {
    pub fn provision_block() {
        todo!()
    }
    pub fn provison_page() {
        todo!()
    }
}

struct ChannelProvisioner {
    luns: [LUN; MEDIA_MANAGER.n_luns],
    n_luns: usize,
    last_lun_picked: usize,
    channel_id: usize,
}

#[derive(Copy, Clone)]
struct LUN {
    // TODO: we make these way too big for starters
    free: RingBuffer<Block, N_BLOCKS_PER_LUN>,
    used: RingBuffer<Block, N_BLOCKS_PER_LUN>,
    partially_used: RingBuffer<BlockWithPageInfo, N_BLOCKS_PER_LUN>,
}

#[derive(Copy, Clone)]
struct Block {
    id: usize,
    plane_id: usize,
}

#[derive(Copy, Clone)]
struct BlockWithPageInfo {
    id: usize,
    plane_id: usize,
    pages: [Page; MEDIA_MANAGER.n_pages],
    n_pages: usize,
}

#[derive(Copy, Clone)]
struct Page {
    dirty: bool,
}

impl ChannelProvisioner {
    fn provision_block(&mut self) -> Result<PhysicalBlockAddress, C_ERR> {
        for _i in 0..MEDIA_MANAGER.n_luns {
            self.last_lun_picked = (self.last_lun_picked + 1) % MEDIA_MANAGER.n_luns;
            let mut lun = self.luns[self.last_lun_picked];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. move the block from free to used
            // 4. return the pba
            if let Some(block) = lun.free.pop() {
                lun.used.push(block);
                let pba = PhysicalBlockAddress {
                    channel: self.channel_id,
                    lun: self.last_lun_picked,
                    plane: block.plane_id as u8,
                    block: block.id,
                };
                return Ok(pba);
            }
        }
        return Err(1);
    }

    fn provison_page(&mut self) -> Result<PhysicalPageAddress, C_ERR> {
        for _i in 0..MEDIA_MANAGER.n_luns {
            self.last_lun_picked = (self.last_lun_picked + 1) % MEDIA_MANAGER.n_luns;
            let mut lun = self.luns[self.last_lun_picked];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. move the block from free to used
            // 4. return the pba

            if let Some(mut block) = lun.partially_used.pop() {
                for (idx, page) in block.pages.iter_mut().enumerate() {
                    if !page.dirty {
                        page.dirty = true;

                        // if it is the last clean page in the block, then move block to used
                        if idx == block.pages.len() - 1 {
                            lun.used.push(Block {
                                id: block.id,
                                plane_id: block.plane_id,
                            });
                        }

                        let ppa = PhysicalPageAddress {
                            channel: self.channel_id,
                            lun: self.last_lun_picked,
                            plane: block.plane_id as u8,
                            block: block.id,
                            page: idx,
                        };
                        return Ok(ppa);
                    }
                }
            }
            // if there are no partially_used page blocks?
            else if let Some(block) = lun.free.pop() {
                let block_with_page_info = BlockWithPageInfo {
                    id: block.id,
                    plane_id: block.plane_id,
                    pages: [Page { dirty: false }; MEDIA_MANAGER.n_pages],
                    n_pages: MEDIA_MANAGER.n_pages,
                };
                lun.partially_used.push(block_with_page_info);
                let ppa = PhysicalPageAddress {
                    channel: self.channel_id,
                    lun: self.last_lun_picked,
                    plane: block.plane_id as u8,
                    block: block.id,
                    page: 0,
                };
                return Ok(ppa);
            }
        }
        return Err(1);
    }
}

// To provision a block we need:
// - It's free
// - It's not bad (not in bb table)

// - Extra:
// - We don't want to provison two blocks in the same lun, since we cannot parallelize I/Os then.
// - Maybe round-robin fashino
