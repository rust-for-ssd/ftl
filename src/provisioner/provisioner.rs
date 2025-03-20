use crate::config;
use crate::{
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
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
pub struct Provisoner {
    pub channel_provisioners: [ChannelProvisioner; config::N_CHANNELS],
    last_channel_provisioned: usize,
}

const fn generate_channel_provisioners<const N: usize>() -> [ChannelProvisioner; N] {
    let mut arr = [ChannelProvisioner::new(0); N];

    let mut i = 0;
    while i < N {
        arr[i].channel_id = i;
        i += 1;
    }
    return arr;
}

impl Provisoner {
    pub const fn new() -> Self {
        Provisoner {
            channel_provisioners: generate_channel_provisioners::<{ config::N_CHANNELS }>(),
            last_channel_provisioned: 0,
        }
    }

    pub fn provision_block(&mut self) -> Result<PhysicalBlockAddress, ProvisionError> {
        // Round robin choose a channel
        self.last_channel_provisioned =
            (self.last_channel_provisioned + 1) % self.channel_provisioners.len();
        let channel_provisoner: &mut ChannelProvisioner =
            &mut self.channel_provisioners[self.last_channel_provisioned];
        channel_provisoner.provision_block()
    }
    pub fn provison_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        self.last_channel_provisioned =
            (self.last_channel_provisioned + 1) % self.channel_provisioners.len();
        let channel_provisoner: &mut ChannelProvisioner =
            &mut self.channel_provisioners[self.last_channel_provisioned];
        channel_provisoner.provison_page()
    }
}

#[derive(Copy, Clone)]
pub struct ChannelProvisioner {
    pub luns: [LUN; config::LUNS_PER_CHANNEL],
    pub last_lun_picked: usize,
    pub channel_id: usize,
}

#[derive(Copy, Clone)]
pub struct LUN {
    pub free: RingBuffer<Block, { config::BLOCKS_PER_LUN }>,
    pub used: RingBuffer<Block, { config::BLOCKS_PER_LUN }>,
    pub partially_used: RingBuffer<BlockWithPageInfo, { config::BLOCKS_PER_LUN }>,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub id: usize,
    pub plane_id: usize,
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
    pub const fn new(channel_id: usize) -> Self {
        ChannelProvisioner {
            luns: [LUN {
                free: RingBuffer::new(), // we assume all 3 lists contain blocks that are not in bbt
                used: RingBuffer::new(),
                partially_used: RingBuffer::new(),
            }; config::LUNS_PER_CHANNEL],
            last_lun_picked: 0,
            channel_id,
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

                lun.used.push(block);
                return Ok(pba);
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
        return Err(ProvisionError::NoFreePage);
    }
}
