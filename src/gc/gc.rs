use crate::{bad_block_table::table::BadBlockTable, core::address::PhysicalBlockAddress, ftl::{GLOBAL_FTL}};

pub struct GarbageCollector {}

impl GarbageCollector {
    pub const fn new() -> Self {
        GarbageCollector {}
    }




    // pub fn mark_as_free(&self, pba: PhysicalBlockAddress) {
    //     GLOBAL_FTL.provisioner.
    // }




}
