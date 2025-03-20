use core::mem::MaybeUninit;

//TODO: we need a function to save the bbt state at some point.

use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
};

use crate::media_manager::operations::{MediaManager, MediaManagerError};

#[derive(PartialEq, Debug)]
pub struct BadBlockTable {
    pub channel_bad_block_tables: [ChannelBadBlockTable; config::N_CHANNELS],
}

impl BadBlockTable {
    pub const fn new() -> Self {
        BadBlockTable {
            channel_bad_block_tables: generate_channel_bbts::<{ config::N_CHANNELS }>(),
        }
    }
    pub fn factory_init<MM: MediaManager>(&mut self) -> Result<(), BadBlockTableError> {
        for ch in self.channel_bad_block_tables.iter_mut() {
            if let Err(e) = ch.factory_init::<MM>() {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn restore_state_from_boot<MM: MediaManager>() -> Result<Self, BadBlockTableError> {
        let mut bbt = BadBlockTable {
            channel_bad_block_tables: unsafe { MaybeUninit::uninit().assume_init() },
        };

        for ch_id in 0..bbt.channel_bad_block_tables.len() {
            let Ok(ch_bbt) = ChannelBadBlockTable::restore_state_from_boot::<MM>(ch_id) else {
                return Err(BadBlockTableError::RestoreTable);
            };
            bbt.channel_bad_block_tables[ch_id] = ch_bbt;
        }

        Ok(bbt)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChannelBadBlockTable {
    pub luns: [LUN; config::LUNS_PER_CHANNEL],
    channel_id: usize,
    current_page: usize,
    version: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LUN {
    pub planes: [Plane; config::PLANES_PER_LUN],
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Plane {
    pub blocks: [BlockStatus; config::BLOCKS_PER_PLANE],
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlockStatus {
    Good,
    Bad,
    Reserved,
}

#[derive(PartialEq, Debug)]
pub enum BadBlockTableError {
    FactoryInitTable,
    RestoreTable,
}

fn factory_init_get_block_status<MM: MediaManager>(pba: &PhysicalBlockAddress) -> BlockStatus {
    if pba.is_reserved() {
        return BlockStatus::Reserved;
    }

    match MM::erase_block(pba) {
        Ok(()) => BlockStatus::Good,
        Err(_) => BlockStatus::Bad,
    }
}

const fn generate_channel_bbts<const N: usize>() -> [ChannelBadBlockTable; N] {
    let mut arr = [ChannelBadBlockTable::new(0); N];

    let mut i = 0;
    while i < N {
        arr[i].channel_id = i;
        i += 1;
    }
    return arr;
}

impl ChannelBadBlockTable {
    pub const fn new(channel_id: usize) -> Self {
        ChannelBadBlockTable {
            luns: [LUN {
                planes: [Plane {
                    blocks: [BlockStatus::Good; config::BLOCKS_PER_PLANE],
                }; config::PLANES_PER_LUN],
            }; config::LUNS_PER_CHANNEL],
            channel_id,
            current_page: 0,
            version: 0,
        }
    }

    fn factory_init<MM: MediaManager>(&mut self) -> Result<(), BadBlockTableError> {
        for (lun_id, lun) in self.luns.iter_mut().enumerate() {
            for (plane_id, plane) in lun.planes.iter_mut().enumerate() {
                for (block_id, block) in plane.blocks.iter_mut().enumerate() {
                    let pba: PhysicalBlockAddress = PhysicalBlockAddress {
                        channel_id: self.channel_id,
                        lun_id,
                        plane_id,
                        block_id,
                    };

                    *block = factory_init_get_block_status::<MM>(&pba);
                }
            }
        }

        if let Err(_) = self.flush::<MM>() {
            return Err(BadBlockTableError::FactoryInitTable);
        } else {
            return Ok(());
        }
    }

    fn flush<MM: MediaManager>(&mut self) -> Result<(), MediaManagerError> {
        self.current_page = (self.current_page + 1) % config::PAGES_PER_BLOCK;
        self.version += 1;

        let ppa = &PhysicalPageAddress {
            channel_id: self.channel_id,
            lun_id: 0,
            plane_id: 0,
            block_id: 0,
            page_id: self.current_page,
        };

        return MM::write_page(ppa);
    }

    // assumption: the bb table can be contained in a single page
    fn restore_state_from_boot<MM: MediaManager>(
        channel_id: usize,
    ) -> Result<Self, BadBlockTableError> {
        let mut latest_version = 0;

        for page in 0..config::PAGES_PER_BLOCK {
            let ppa = &PhysicalPageAddress {
                channel_id,
                lun_id: 0,
                plane_id: 0,
                block_id: 0,
                page_id: page,
            };

            if let Ok(table_from_disk) = MM::read_page::<ChannelBadBlockTable>(ppa) {
                if latest_version < table_from_disk.version {
                    latest_version = table_from_disk.version;
                } else {
                    return Ok(table_from_disk);
                }
            }
        }

        return Err(BadBlockTableError::RestoreTable);
    }

    pub fn get_block_status(&self, pba: &PhysicalBlockAddress) -> BlockStatus {
        if pba.is_reserved() {
            return BlockStatus::Reserved;
        }

        let lun = self.luns[pba.lun_id];
        let plane = lun.planes[pba.plane_id as usize];
        return plane.blocks[pba.block_id];
    }
}
