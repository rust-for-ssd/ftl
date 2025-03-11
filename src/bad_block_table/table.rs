use crate::media_manager::stub::{MediaManger, PhysicalBlockAddress};

struct BadBlockTable<'c> {
    channels: &'c mut [Channel<'c>],
    size: usize,
    n_bad_blocks: usize,
}

struct Channel<'lun> {
    luns: &'lun mut [LUN<'lun>],
    n_parallel_units: usize,
    // id: usize,
}

struct LUN<'p> {
    planes: &'p mut [Plane<'p>],
    n_chunks: usize,
    // id: usize,
}

struct Plane<'b> {
    blocks: &'b mut [IsBadBlock],
    // blocks: &'b mut [Block<'b>],
    n_planes: u8,
    // id: usize,
}

// TODO: find a better name?
type IsBadBlock = bool;

// struct Block<'p> {
//     pages: &'p mut [bool], // WARN: we might not need this
//     n_plane_blocks: usize,
//     // id: usize,
// }

impl<'c> BadBlockTable<'c> {
    fn new() -> Self {
        // we need the dimentions, maybe we get this from the media manager?
        // we need the place to store the table, this can be static or on the heap?
        todo!()
    }

    fn init(&mut self) {
        for (channel_id, channel) in self.channels.iter_mut().enumerate() {
            for (lun_id, lun) in channel.luns.iter_mut().enumerate() {
                for (plane_id, plane) in lun.planes.iter_mut().enumerate() {
                    for (block_id, block) in plane.blocks.iter_mut().enumerate() {
                        let pba: PhysicalBlockAddress = PhysicalBlockAddress {
                            channel: channel_id,
                            lun: lun_id,
                            plane: plane_id as u8,
                            block: block_id,
                        };

                        *block = is_block_bad(&pba);
                    }
                }
            }
        }
    }
}

fn is_block_bad(pba: &PhysicalBlockAddress) -> IsBadBlock {
    match MediaManger::erase_block(pba) {
        Ok(()) => false,
        Err(_) => true,
    }
}
