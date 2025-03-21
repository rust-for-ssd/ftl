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
pub struct Provisioner {
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

impl Provisioner {
    pub const fn new() -> Self {
        Provisioner {
            channel_provisioners: generate_channel_provisioners::<{ config::N_CHANNELS }>(),
            last_channel_provisioned: 0,
        }
    }

    pub fn provision_block(&mut self) -> Result<PhysicalBlockAddress, ProvisionError> {
        // Round robin choose a channel
        for i in 0..self.channel_provisioners.len() {
            let ch_idx = (self.last_channel_provisioned + 1 + i) % self.channel_provisioners.len();
            let channel_provisoner: &mut ChannelProvisioner =
                &mut self.channel_provisioners[ch_idx];
            if let Ok(pba) = channel_provisoner.provision_block() {
                self.last_channel_provisioned = ch_idx;
                return Ok(pba);
            }
        }
        Err(ProvisionError::NoFreeBlock)
    }

    pub fn provision_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        for i in 0..self.channel_provisioners.len() {
            let ch_idx = (self.last_channel_provisioned + 1 + i) % self.channel_provisioners.len();
            let channel_provisoner: &mut ChannelProvisioner =
                &mut self.channel_provisioners[ch_idx];
            if let Ok(ppa) = channel_provisoner.provision_page() {
                self.last_channel_provisioned = ch_idx;
                return Ok(ppa);
            }
        }
        Err(ProvisionError::NoFreePage)
    }
}

#[derive(Copy, Clone)]
pub struct ChannelProvisioner {
    // TODO: should contain a count of free blocks as to not try to provision if there are none.
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
pub struct BlockWithPageInfo {
    pub id: usize,
    pub plane_id: usize,
    pub pages: [Page; config::PAGES_PER_BLOCK],
}

#[derive(Copy, Clone, PartialEq)]
pub enum Page {
    InUse,
    Free,
}

#[derive(Debug, PartialEq)]
pub enum ProvisionError<'s> {
    NoFreeBlock,
    NoFreePage,
    BlockErr(&'s str),
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
        for i in 0..self.luns.len() {
            let lun_idx = (self.last_lun_picked + 1 + i) % self.luns.len();
            let lun = &mut self.luns[lun_idx];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. check if it is bad
            // 4. move the block from free to used
            // 5. return the pba

            if let Some(block) = lun.free.pop() {
                let pba = PhysicalBlockAddress {
                    channel_id: self.channel_id,
                    lun_id: self.last_lun_picked,
                    plane_id: block.plane_id,
                    block_id: block.id,
                };

                let Ok(()) = lun.used.push(block) else {
                    return Err(ProvisionError::BlockErr("Could not push block to used."));
                };
                self.last_lun_picked = lun_idx;
                return Ok(pba);
            }
        }
        return Err(ProvisionError::NoFreeBlock);
    }

    pub fn provision_page(&mut self) -> Result<PhysicalPageAddress, ProvisionError> {
        for i in 0..self.luns.len() {
            let lun_idx = (self.last_lun_picked + 1 + i) % self.luns.len();
            let lun = &mut self.luns[lun_idx];
            // 1. pick a lun
            // 2. get a free block from the lun if there is any
            // 3. move the block from free to used
            // 4. return the pba

            if let Some(mut block) = lun.partially_used.pop() {
                // TODO: can the page be reserved?? if so, then it should be handled.
                for (idx, page) in block.pages.iter_mut().enumerate() {
                    if *page == Page::Free {
                        *page = Page::InUse;

                        // if it is the last page not in use in the block, then move block to used
                        if idx == block.pages.len() - 1 {
                            let Ok(()) = lun.used.push(Block {
                                id: block.id,
                                plane_id: block.plane_id,
                            }) else {
                                return Err(ProvisionError::BlockErr(
                                    "Could not push block to used.",
                                ));
                            };
                        } else {
                            let Ok(()) = lun.partially_used.push(block) else {
                                return Err(ProvisionError::BlockErr(
                                    "Could not push block to partially used.",
                                ));
                            };
                        }

                        let ppa = PhysicalPageAddress {
                            channel_id: self.channel_id,
                            lun_id: lun_idx,
                            plane_id: block.plane_id,
                            block_id: block.id,
                            page_id: idx,
                        };
                        self.last_lun_picked = lun_idx;
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
                    pages: [Page::Free; config::PAGES_PER_BLOCK],
                };
                block_with_page_info.pages[0] = Page::InUse;

                let Ok(()) = lun.partially_used.push(block_with_page_info) else {
                    return Err(ProvisionError::BlockErr(
                        "Could not push block to partially used.",
                    ));
                };
                let ppa = PhysicalPageAddress {
                    channel_id: self.channel_id,
                    lun_id: self.last_lun_picked,
                    plane_id: block.plane_id,
                    block_id: block.id,
                    page_id: 0,
                };
                self.last_lun_picked = lun_idx;
                return Ok(ppa);
            }
        }
        return Err(ProvisionError::NoFreePage);
    }
}
