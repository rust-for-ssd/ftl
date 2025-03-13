use core::array::from_fn;
use crate::bad_block_table::table::ChannelBadBlockTable;
use crate::config;
use crate::{
    bad_block_table::table::BlockStatus,
    media_manager::stub::{ PhysicalBlockAddress, PhysicalPageAddress },
    utils::ring_buffer::RingBuffer,
};

// Page provison: gives a physical page adress (ppa) to an available page
// - To provision a block we need:
// - It's free
// - It's not bad (not in bb table)
// - It's not reserved

// - Extra:
// - We don't want to provison two blocks in the same lun, since we cannot parallelize I/Os then.
// - Maybe round-robin fashion
pub struct GlobalProvisoner {
    channel_provisioners: [ChannelProvisioner; config::N_CHANNELS],
    last_channel_provisioned: usize,
}

impl GlobalProvisoner {
    pub fn new(bbts: [ChannelBadBlockTable; config::N_CHANNELS]) -> Self {
        GlobalProvisoner{
            channel_provisioners: from_fn(|id|ChannelProvisioner::new(id, bbts[id])),
            last_channel_provisioned: 0,
        }
    }

    pub fn provision_block(&mut self) -> Result<PhysicalBlockAddress, ProvisionError> {
        // Round robin choose a channel
        self.last_channel_provisioned = (self.last_channel_provisioned + 1) % self.channel_provisioners.len();
        let channel_provisoner: &mut ChannelProvisioner = &mut self.channel_provisioners[self.last_channel_provisioned];
        channel_provisoner.provision_block()
    }
    pub fn provison_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        self.last_channel_provisioned = (self.last_channel_provisioned + 1) % self.channel_provisioners.len();
        let channel_provisoner: &mut ChannelProvisioner = &mut self.channel_provisioners[self.last_channel_provisioned];
        channel_provisoner.provison_page()
    }
}

struct ChannelProvisioner {
    luns: [LUN; config::LUNS_PER_CHANNEL],
    last_lun_picked: usize,
    channel_id: usize,
    bbt: ChannelBadBlockTable,
}

#[derive(Copy, Clone)]
struct LUN {
    free: RingBuffer<Block, { config::BLOCKS_PER_LUN }>,
    used: RingBuffer<Block, { config::BLOCKS_PER_LUN }>,
    partially_used: RingBuffer<BlockWithPageInfo, { config::BLOCKS_PER_LUN }>,
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
    pages: [Page; config::PAGES_PER_BLOCK],
}

#[derive(Copy, Clone)]
struct Page {
    in_use: bool,
}

pub enum ProvisionError {
    NoFreeBlock,
    NoFreePage,
}

impl ChannelProvisioner {
    pub fn new(channel_id: usize, bbt: ChannelBadBlockTable) -> Self {
        ChannelProvisioner{
            luns: [LUN{
                free: RingBuffer::new(),
                used: RingBuffer::new(),
                partially_used: RingBuffer::new(),
            }; config::LUNS_PER_CHANNEL],
            last_lun_picked: 0,
            channel_id,
            bbt
        }
    }

    pub fn provision_block(&mut self) -> Result<PhysicalBlockAddress, ProvisionError> {
        for _i in 0..self.luns.len() {
            self.last_lun_picked = (self.last_lun_picked + 1) % self.luns.len();
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
                    plane: block.plane_id,
                    block: block.id,
                };

                if let BlockStatus::Good = self.bbt.get_block_status(&pba) {
                    lun.used.push(block);
                    return Ok(pba);
                }
            }
        }
        return Err(ProvisionError::NoFreeBlock);
    }

    pub fn provison_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        for _i in 0..self.luns.len() {
            self.last_lun_picked = (self.last_lun_picked + 1) % self.luns.len();
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
                            plane: block.plane_id,
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
                    plane: block.plane_id,
                    block: block.id,
                };

                if let BlockStatus::Good = self.bbt.get_block_status(&pba) {
                    let mut block_with_page_info = BlockWithPageInfo {
                        id: block.id,
                        plane_id: block.plane_id,
                        pages: [Page { in_use: false }; config::PAGES_PER_BLOCK],
                    };
                    block_with_page_info.pages[0] = Page { in_use: true };

                    lun.partially_used.push(block_with_page_info);
                    let ppa = PhysicalPageAddress {
                        channel: self.channel_id,
                        lun: self.last_lun_picked,
                        plane: block.plane_id,
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


