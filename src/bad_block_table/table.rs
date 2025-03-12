use crate::media_manager::stub::{
    MEDIA_MANAGER, MediaManager, MediaManagerError, PhysicalBlockAddress,
    PhysicalBlockAddressError, PhysicalPageAddress,
};

#[derive(Copy, Clone)]
pub struct ChannelBadBlockTable {
    n_bad_blocks: usize,
    channel: Channel,
    channel_id: usize,
    current_page: usize,
    version: usize,
}

#[derive(Copy, Clone)]
struct Channel {
    luns: [LUN; MEDIA_MANAGER.n_luns],
    n_luns: usize,
}

#[derive(Copy, Clone)]
struct LUN {
    planes: [Plane; MEDIA_MANAGER.n_planes],
    n_planes: usize,
}

#[derive(Copy, Clone)]
struct Plane {
    blocks: [IsBadBlock; MEDIA_MANAGER.n_blocks_per_plane],
    n_blocks: usize,
}

// TODO: find a better name?
type IsBadBlock = bool;
pub const IS_NOT_BAD_BLOCK: bool = true;

pub enum BadBlockTableError {
    FactoryInitTable,
    RestoreTable,
}

fn factory_init_is_block_bad(
    pba: &PhysicalBlockAddress,
) -> Result<IsBadBlock, PhysicalBlockAddressError> {
    if pba.is_reserved() {
        return Err(PhysicalBlockAddressError::Reserved);
    }

    match MediaManager::erase_block(pba) {
        Ok(()) => Ok(false),
        Err(_) => Ok(true),
    }
}

impl ChannelBadBlockTable {
    fn new(channel_id: usize, n_luns: usize, n_planes: usize, n_blocks: usize) -> Self {
        let channel = Channel {
            luns: [LUN {
                n_planes,
                planes: [Plane {
                    n_blocks,
                    blocks: [false; MEDIA_MANAGER.n_blocks_per_plane],
                }; MEDIA_MANAGER.n_planes],
            }; MEDIA_MANAGER.n_luns],
            n_luns,
        };

        ChannelBadBlockTable {
            n_bad_blocks: 0,
            channel,
            channel_id,
            current_page: 0,
            version: 0,
        }
    }

    fn factory_init(&mut self) -> Result<(), BadBlockTableError> {
        for (lun_id, lun) in self.channel.luns.iter_mut().enumerate() {
            for (plane_id, plane) in lun.planes.iter_mut().enumerate() {
                for (block_id, block) in plane.blocks.iter_mut().enumerate() {
                    let pba: PhysicalBlockAddress = PhysicalBlockAddress {
                        channel: self.channel_id,
                        lun: lun_id,
                        plane: plane_id as u8,
                        block: block_id,
                    };

                    if let Ok(is_bad) = factory_init_is_block_bad(&pba) {
                        *block = is_bad;
                    } else {
                        *block = false;
                    }
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
        self.current_page = (self.current_page + 1) % MEDIA_MANAGER.n_pages;
        self.version += 1;

        let ppa = &PhysicalPageAddress {
            channel: self.channel_id,
            lun: 0,
            plane: 0,
            block: 0,
            page: self.current_page,
        };

        return MediaManager::write_page(ppa);
    }

    fn restore_state_from_boot(&mut self) -> Result<Self, BadBlockTableError> {
        // assumption: the bb table can be contained in a single page

        let mut latest_version = 0;

        for page in 0..MEDIA_MANAGER.n_pages {
            let ppa = &PhysicalPageAddress {
                channel: self.channel_id,
                lun: 0,
                plane: 0,
                block: 0,
                page: page,
            };

            if let Ok(table_from_disk) = MediaManager::read_page::<ChannelBadBlockTable>(ppa) {
                //throws Err is unpack fails
                if latest_version < table_from_disk.version {
                    latest_version = table_from_disk.version;
                } else {
                    return Ok(table_from_disk);
                }
            }
        }

        return Err(BadBlockTableError::RestoreTable);
    }

    pub fn is_block_bad(
        &self,
        pba: &PhysicalBlockAddress,
    ) -> Result<IsBadBlock, PhysicalBlockAddressError> {
        if pba.is_reserved() {
            return Err(PhysicalBlockAddressError::Reserved);
        }

        let lun = self.channel.luns[pba.lun];
        let plane = lun.planes[pba.plane as usize];
        let is_bad = plane.blocks[pba.block];
        return Ok(is_bad);
    }
}
