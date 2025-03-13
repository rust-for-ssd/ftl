pub const N_CHANNELS: usize = 24;
pub const LUNS_PER_CHANNEL: usize = 4;
pub const PLANES_PER_LUN: usize = 1;
pub const BLOCKS_PER_PLANE: usize = 1024;
pub const BLOCKS_PER_LUN: usize = BLOCKS_PER_PLANE * PLANES_PER_LUN;
pub const PAGES_PER_BLOCK: usize = 16;
