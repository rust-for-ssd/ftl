struct BadBlockTable<'c> {
    channels: &'c mut [Channel<'c>],
    size: usize,
    n_bad_blocks: usize,
}

struct Channel<'pu> {
    parallel_units: &'pu mut [ParallelUnit<'pu>],
    n_parallel_units: usize,
    id: usize
}

struct ParallelUnit<'c> {
    chunks: &'c mut [Chunk<'c>],
    n_chunks: usize,
    id: usize
}

struct Chunk<'pb> {
    plane_blocks: &'pb mut [PlaneBlock],
    n_plane_blocks: usize,
    id: usize
}

struct PlaneBlock {
    n_planes: u8,
    id: usize
}

impl BadBlockTable<'_> {
    fn init() -> Self {
        todo!()
    }
}

impl Channel<'_> {
    fn init() -> Self {
        todo!()
    }
}

impl ParallelUnit<'_> {
    fn init() -> Self {
        todo!()
    }
}

impl Chunk<'_> {
    fn check_bad_blocks(self) -> Self {
        // for plane_block in
        todo!()
    }
}

impl PlaneBlock {
    fn check_if_bad(self, channel_id, lun_id, block_id) -> bool {
        let page = 0;
        // channel
        // lun
        // block
        // 

        todo!()
    }
}
