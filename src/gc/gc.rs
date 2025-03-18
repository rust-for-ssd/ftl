use crate::{bad_block_table::table::GlobalBadBlockTable, core::address::PhysicalBlockAddress, ftl::{GLOBAL_FTL}};

pub struct GarbageCollector {}

impl GarbageCollector {
    pub const fn new() -> Self {
        GarbageCollector {}
    }
    pub fn free_block(&self, pba: PhysicalBlockAddress) {
        // GLOBAL_FTL.
    }
}
