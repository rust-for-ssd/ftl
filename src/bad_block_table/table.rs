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
    blocks: [BadBlockEntry; MEDIA_MANAGER.n_blocks_per_plane],
    n_blocks: usize,
}

#[derive(Copy, Clone)]
pub enum BadBlockEntry {
    Good,
    Bad,
    Reserved,
}

pub enum BadBlockTableError {
    FactoryInitTable,
    RestoreTable,
}

fn factory_init_get_entry_type(pba: &PhysicalBlockAddress) -> BadBlockEntry {
    if pba.is_reserved() {
        return BadBlockEntry::Reserved;
    }

    match MediaManager::erase_block(pba) {
        Ok(()) => BadBlockEntry::Good,
        Err(_) => BadBlockEntry::Bad,
    }
}

impl ChannelBadBlockTable {
    pub fn new(channel_id: usize, n_luns: usize, n_planes: usize, n_blocks: usize) -> Self {
        let channel = Channel {
            luns: [LUN {
                n_planes,
                planes: [Plane {
                    n_blocks,
                    blocks: [BadBlockEntry::Good; MEDIA_MANAGER.n_blocks_per_plane],
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

                    *block = factory_init_get_entry_type(&pba);
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
                if latest_version < table_from_disk.version {
                    latest_version = table_from_disk.version;
                } else {
                    return Ok(table_from_disk);
                }
            }
        }

        return Err(BadBlockTableError::RestoreTable);
    }

    pub fn get_block_type(&self, pba: &PhysicalBlockAddress) -> BadBlockEntry {
        if pba.is_reserved() {
            return BadBlockEntry::Reserved;
        }

        let lun = self.channel.luns[pba.lun];
        let plane = lun.planes[pba.plane as usize];
        return plane.blocks[pba.block];
    }
}
