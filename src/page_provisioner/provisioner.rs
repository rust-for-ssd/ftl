// Page provison: gives a physical page adress (ppa) to an available page

use crate::{
    bad_block_table::table::BadBlockEntry,
    media_manager::stub::{
        MEDIA_MANAGER, N_BLOCKS_PER_LUN, PhysicalBlockAddress, PhysicalPageAddress,
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

use crate::bad_block_table::table::ChannelBadBlockTable;

struct ChannelProvisioner {
    luns: [LUN; MEDIA_MANAGER.n_luns],
    n_luns: usize,
    last_lun_picked: usize,
    channel_id: usize,
    bbt: ChannelBadBlockTable,
}

#[derive(Copy, Clone)]
struct LUN {
    // TODO: we make these way too big for starters
    free: RingBuffer<Block, N_BLOCKS_PER_LUN>,
    used: RingBuffer<Block, N_BLOCKS_PER_LUN>,
    partially_used: RingBuffer<BlockWithPageInfo, N_BLOCKS_PER_LUN>,
    // partial_page_count: usize,
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
    in_use: bool,
}

enum ProvisionError {
    NoFreeBlock,
    NoFreePage,
}

impl ChannelProvisioner {
    fn provision_block(&mut self) -> Result<PhysicalBlockAddress, ProvisionError> {
        for _i in 0..MEDIA_MANAGER.n_luns {
            self.last_lun_picked = (self.last_lun_picked + 1) % MEDIA_MANAGER.n_luns;
            let mut lun = self.luns[self.last_lun_picked];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. check if it is bad
            // 4. move the block from free to used
            // 5. return the pba
            if let Some(block) = lun.free.pop() {
                let pba = PhysicalBlockAddress {
                    channel: self.channel_id,
                    lun: self.last_lun_picked,
                    plane: block.plane_id as u8,
                    block: block.id,
                };

                if let BadBlockEntry::Good = self.bbt.get_block_type(&pba) {
                    lun.used.push(block);
                    return Ok(pba);
                }
            }
        }
        return Err(ProvisionError::NoFreeBlock);
    }

    fn provison_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        for _i in 0..MEDIA_MANAGER.n_luns {
            self.last_lun_picked = (self.last_lun_picked + 1) % MEDIA_MANAGER.n_luns;
            let lun_id = self.last_lun_picked;
            let mut lun = self.luns[lun_id];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. move the block from free to used
            // 4. return the pba

            if let Some(mut block) = lun.partially_used.pop() {
                // TODO: can the page be reserved?? if so, then it should be handled.
                for (idx, page) in block.pages.iter_mut().enumerate() {
                    if !page.in_use {
                        page.in_use = true;

                        // if it is the last page not in use in the block, then move block to used
                        if idx == block.pages.len() - 1 {
                            lun.used.push(Block {
                                id: block.id,
                                plane_id: block.plane_id,
                            });
                        } else {
                            lun.partially_used.push(block);
                        }

                        let ppa = PhysicalPageAddress {
                            channel: self.channel_id,
                            lun: lun_id,
                            plane: block.plane_id as u8,
                            block: block.id,
                            page: idx,
                        };
                        return Ok(ppa);
                    }
                }
            }
            // if there are no partially_used page blocks?
            // NOTE: this gets a free block from the same lun, instead of checking for another partially_used block in another lun
            // This might not be the best behaviour, depending on if it is better to check other lun partially_used
            else if let Some(block) = lun.free.pop() {
                let pba = PhysicalBlockAddress {
                    channel: self.channel_id,
                    lun: lun_id,
                    plane: block.plane_id as u8,
                    block: block.id,
                };

                if let BadBlockEntry::Good = self.bbt.get_block_type(&pba) {
                    let mut block_with_page_info = BlockWithPageInfo {
                        id: block.id,
                        plane_id: block.plane_id,
                        pages: [Page { in_use: false }; MEDIA_MANAGER.n_pages],
                        n_pages: MEDIA_MANAGER.n_pages,
                    };
                    block_with_page_info.pages[0] = Page { in_use: true };

                    // lun.partial_page_count += block_with_page_info.pages.len() - 1;
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
        }
        return Err(ProvisionError::NoFreePage);
    }
}

// To provision a block we need:
// - It's free
// - It's not bad (not in bb table)
// - It's not reserved

// - Extra:
// - We don't want to provison two blocks in the same lun, since we cannot parallelize I/Os then.
// - Maybe round-robin fashino
