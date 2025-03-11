struct BadBlockTable<'c> {
    channels: &'c mut [Channel<'c>],
    size: usize,
    n_bad_blocks: usize,
}

struct Channel<'pu> {
    parallel_units: &'pu mut [ParallelUnit<'pu>],
    n_parallel_units: usize,
    id: usize,
}

struct ParallelUnit<'c> {
    chunks: &'c mut [Chunk<'c>],
    n_chunks: usize,
    id: usize,
}

struct Chunk<'pb> {
    plane_blocks: &'pb mut [PlaneBlock],
    n_plane_blocks: usize,
    id: usize,
}

struct PlaneBlock {
    n_planes: u8,
    id: usize,
}

impl<'c> BadBlockTable<'c> {
    fn init(&mut self) {
        for channel in &mut *self.channels {
            for pu in &mut *channel.parallel_units {
                for chunk in &mut *pu.chunks {
                    for plane_block in &mut *chunk.plane_blocks {
                        let is_bad = plane_block.health_check();
                        if is_bad {
                            // Mark as bad somehow
                        }
                    }
                }
            }
        }
    }
}

impl PlaneBlock {
    fn health_check(&self) -> bool {
        // do a call to the media manager
        false
    }
}