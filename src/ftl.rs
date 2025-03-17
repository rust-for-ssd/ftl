use crate::config;
use crate::core::address::{LogicalPageAddress};
use crate::logical_physical_address::mapper::L2P_Mapper;
use crate::page_provisioner::provisioner::GlobalProvisoner;
use crate::bad_block_table::table::ChannelBadBlockTable;

pub struct FTL {
    l2p_map: L2P_Mapper,
    provisioner: GlobalProvisoner,
    bb_tables: [ChannelBadBlockTable; config::N_CHANNELS], 

}

impl FTL {

    pub fn new() -> Self {
        FTL
    }

    pub fn read_page(&self, lpa: LogicalPageAddress) -> () {

        todo!()
    }

    pub fn write_page(&self, lpa: LogicalPageAddress) -> () {
        todo!()
    }
}