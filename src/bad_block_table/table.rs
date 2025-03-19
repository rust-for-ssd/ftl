use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::{FTL, GLOBAL_FTL},
    media_manager::stub::{MediaManager, MediaManagerError},
};

use crate::media_manager::stub::MediaOperations;

pub struct BadBlockTable {
    pub channel_bad_block_tables: [ChannelBadBlockTable; config::N_CHANNELS],
}

impl BadBlockTable {
    pub const fn new() -> Self {
        BadBlockTable {
            channel_bad_block_tables: generate_channel_bbts::<{ config::N_CHANNELS }>(),
        }
    }
    pub fn factory_init(&mut self) -> Result<(), BadBlockTableError> {
        for ch in self.channel_bad_block_tables.iter_mut() {
            if let Err(e) = ch.factory_init() {
                return Err(e);
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct ChannelBadBlockTable {
    n_bad_blocks: usize,
    pub luns: [LUN; config::LUNS_PER_CHANNEL],
    channel_id: usize,
    current_page: usize,
    version: usize,
}

#[derive(Copy, Clone)]
pub struct LUN {
    pub planes: [Plane; config::PLANES_PER_LUN],
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub blocks: [BlockStatus; config::BLOCKS_PER_PLANE],
}

#[derive(Copy, Clone, PartialEq)]
pub enum BlockStatus {
    Good,
    Bad,
    Reserved,
}

pub enum BadBlockTableError {
    FactoryInitTable,
    RestoreTable,
}

fn factory_init_get_block_status(pba: &PhysicalBlockAddress) -> BlockStatus {
    if pba.is_reserved() {
        return BlockStatus::Reserved;
    }

    match GLOBAL_FTL.mm.read_block(pba) {
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
            n_bad_blocks: 0,
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

    fn factory_init(&mut self) -> Result<(), BadBlockTableError> {
        for (lun_id, lun) in self.luns.iter_mut().enumerate() {
            for (plane_id, plane) in lun.planes.iter_mut().enumerate() {
                for (block_id, block) in plane.blocks.iter_mut().enumerate() {
                    let pba: PhysicalBlockAddress = PhysicalBlockAddress {
                        channel: self.channel_id,
                        lun: lun_id,
                        plane: plane_id,
                        block: block_id,
                    };

                    *block = factory_init_get_block_status(&pba);
                }
            }
        }

        if let Err(_) = self.flush() {
            return Err(BadBlockTableError::FactoryInitTable);
        } else {
            return Ok(());
        }
    }

    fn flush(&mut self) -> Result<(), MediaManagerError> {
        self.current_page = (self.current_page + 1) % config::PAGES_PER_BLOCK;
        self.version += 1;

        let ppa = &PhysicalPageAddress {
            channel: self.channel_id,
            lun: 0,
            plane: 0,
            block: 0,
            page: self.current_page,
        };

        return GLOBAL_FTL.mm.read_page(ppa);
    }

    // assumption: the bb table can be contained in a single page
    fn restore_state_from_boot(channel_id: usize) -> Result<Self, BadBlockTableError> {
        let mut latest_version = 0;

        for page in 0..config::PAGES_PER_BLOCK {
            let ppa = &PhysicalPageAddress {
                channel: channel_id,
                lun: 0,
                plane: 0,
                block: 0,
                page: page,
            };

            if let Ok(table_from_disk) = GLOBAL_FTL.mm.read_page::<ChannelBadBlockTable>(ppa) {
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

        let lun = self.luns[pba.lun];
        let plane = lun.planes[pba.plane as usize];
        return plane.blocks[pba.block];
    }
}
