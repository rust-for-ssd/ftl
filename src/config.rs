// Configurable
pub const N_CHANNELS: usize = 8;
pub const LUNS_PER_CHANNEL: usize = 4;
pub const PLANES_PER_LUN: usize = 1;
pub const BLOCKS_PER_PLANE: usize = 64; // 1024 might be realistic number, need to update qemu config
pub const PAGES_PER_BLOCK: usize = 64; // 64 to 512 pages per block
pub const BYTES_PER_PAGE: usize = 4096; // 4 to 32 kilobytes per page -- this does not affect the FTL size!

//Derivatives
pub const TOTAL_BLOCKS: usize = N_CHANNELS * LUNS_PER_CHANNEL * BLOCKS_PER_LUN;
pub const TOTAL_PAGES: usize = TOTAL_BLOCKS * PAGES_PER_BLOCK;
pub const BLOCKS_PER_LUN: usize = BLOCKS_PER_PLANE * PLANES_PER_LUN;
pub const TOTAL_BYTES: usize = N_CHANNELS
    * LUNS_PER_CHANNEL
    * PLANES_PER_LUN
    * BLOCKS_PER_LUN
    * PAGES_PER_BLOCK
    * BYTES_PER_PAGE;
pub const TOTAL_MB: f64 = TOTAL_BYTES as f64 / (1024.000 * 1024.000);
