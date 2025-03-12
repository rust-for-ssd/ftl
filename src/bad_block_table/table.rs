use crate::media_manager::stub::{
    C_ERR, MEDIA_MANAGER, MediaManger, PhysicalBlockAddress, PhysicalPageAddress,
};

#[derive(Copy, Clone)]
struct BadBlockChannelTable {
    n_bad_blocks: usize,
    channel: Channel,
    channel_id: usize,
    current_page: usize,
    version: usize
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
    blocks: [IsBadBlock; MEDIA_MANAGER.n_blocks],
    n_blocks: usize,
}

// TODO: find a better name?
type IsBadBlock = bool;

fn is_block_bad(pba: &PhysicalBlockAddress) -> IsBadBlock {
    if pba.is_reserved() {
        return false;
    }

    match MediaManger::erase_block(pba) {
        Ok(()) => false,
        Err(_) => true,
    }
}

impl BadBlockChannelTable {
    fn new(channel_id: usize, n_luns: usize, n_planes: usize, n_blocks: usize) -> Self {
        let channel = Channel {
            luns: [LUN {
                n_planes,
                planes: [Plane {
                    n_blocks,
                    blocks: [false; MEDIA_MANAGER.n_blocks],
                }; MEDIA_MANAGER.n_planes],
            }; MEDIA_MANAGER.n_luns],
            n_luns,
        };

        BadBlockChannelTable {
            n_bad_blocks: 0,
            channel,
            channel_id,
            current_page: 0,
            version: 0,
        }
    }

    fn factory_init(&mut self) -> Result<(), C_ERR> {
        for (lun_id, lun) in self.channel.luns.iter_mut().enumerate() {
            for (plane_id, plane) in lun.planes.iter_mut().enumerate() {
                for (block_id, block) in plane.blocks.iter_mut().enumerate() {
                    let pba: PhysicalBlockAddress = PhysicalBlockAddress {
                        channel: self.channel_id,
                        lun: lun_id,
                        plane: plane_id as u8,
                        block: block_id,
                    };

                    *block = is_block_bad(&pba);
                }
            }
        }

        return self.flush();
    }

    fn flush(&mut self) -> Result<(), C_ERR> {
        self.current_page = (self.current_page + 1) % MEDIA_MANAGER.n_pages;
        self.version += 1;

        let ppa = &PhysicalPageAddress {
            channel: self.channel_id,
            lun: 0,
            plane: 0,
            block: 0,
            page: self.current_page,
        };

        return MediaManger::write_page(ppa);
    }

    fn restore_state_from_boot(&mut self) -> Result<Self, C_ERR> {
        // assumption: the bb table can be contained in a single page

        let latest_version = 0; 

        for page in 0..MEDIA_MANAGER.n_pages {

            let ppa = &PhysicalPageAddress {
                channel: self.channel_id,
                lun: 0,
                plane: 0,
                block: 0,
                page: page,
            };

            let table_from_disk = MediaManger::read_page(ppa)? as BadBlockChannelTable; //throws Err is unpack fails

            if (latest_version < table_from_disk.version) {
                latest_version = table_from_disk.version;
            } else { 
                return Ok(table_from_disk);
            }
        }

        Err(1)
        
    }
}


