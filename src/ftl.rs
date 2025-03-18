use crate::bad_block_table::table::{ChannelBadBlockTable, GlobalBadBlockTable};
use crate::config;
use crate::core::address::LogicalPageAddress;
use crate::gc::gc::GarbageCollector;
use crate::logical_physical_address::mapper::L2P_Mapper;
use crate::page_provisioner::provisioner::GlobalProvisoner;

pub struct FTL {
    pub l2p_map: L2P_Mapper,
    pub provisioner: GlobalProvisoner,
    pub bbt: GlobalBadBlockTable,
    pub gc: GarbageCollector,
}

pub static GLOBAL_FTL : FTL = FTL::new();

impl FTL {
    pub const fn new() -> Self {
        FTL {
            l2p_map: L2P_Mapper::new(),
            provisioner: GlobalProvisoner::new(),
            bbt: GlobalBadBlockTable::new(),
            gc : GarbageCollector::new(), 
        }
    }

    pub fn init() { 
        // Factory init bbt
        // Add good blocks from bbt to free list in provisioner
        // 
    }

    pub fn read_page(&self, lpa: LogicalPageAddress) -> () {
        todo!()
    }

    pub fn write_page(&self, lpa: LogicalPageAddress) -> () {
        todo!()
    }
}
