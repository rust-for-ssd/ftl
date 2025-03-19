use crate::bad_block_table::table::{BadBlockTable, BlockStatus, ChannelBadBlockTable};
use crate::config;
use crate::core::address::{LogicalPageAddress, PhysicalPageAddress};
use crate::gc::gc::GarbageCollector;
use crate::logical_physical_address::mapper::L2P_Mapper;
use crate::media_manager::operations::{MediaManagerError, MediaOperations};
use crate::media_manager::stub::MediaManager;
use crate::page_provisioner::provisioner::{self, Block, Provisoner};

pub struct FTL<MediaManager: MediaOperations> {
    pub l2p_map: L2P_Mapper,
    pub provisioner: Provisoner,
    pub bbt: BadBlockTable,
    pub gc: GarbageCollector,
    pub mm: MediaManager,
}

const MM: MediaManager = MediaManager::new();
pub static GLOBAL_FTL: FTL<MediaManager> = FTL::new(MM);

impl<MediaManager: MediaOperations> FTL<MediaManager> {
    pub const fn new(mm: MediaManager) -> Self {
        FTL {
            l2p_map: L2P_Mapper::new(),
            provisioner: Provisoner::new(),
            bbt: BadBlockTable::new(),
            gc: GarbageCollector::new(),
            mm,
        }
    }

    pub fn init(&mut self) -> () {
        // Factory init bbt
        self.bbt.factory_init();

        // Add good blocks from bbt to free list in provisioner
        for (channel_idx, ch) in self.bbt.channel_bad_block_tables.iter().enumerate() {
            for (lun_idx, lun) in ch.luns.iter().enumerate() {
                for (plane_idx, plane) in lun.planes.iter().enumerate() {
                    for (block_idx, block_status) in plane.blocks.iter().enumerate() {
                        if *block_status == BlockStatus::Good {
                            // TODO make this a method in the provisoner and set fields to private
                            self.provisioner.channel_provisioners[channel_idx].luns[lun_idx]
                                .free
                                .push(Block {
                                    id: block_idx,
                                    plane_id: plane_idx,
                                })
                        }
                    }
                }
            }
        }
    }

    pub fn read_page(&self, lpa: LogicalPageAddress) -> Result<PageContent, FTL_ERR> {
        let Some(ppa) = self.l2p_map.get_physical_address(lpa) else {
            return Err(FTL_ERR::READ_PAGE);
        };

        let Ok(content): Result<PageContent, MediaManagerError> =
            // self.mm.read_page(&PhysicalPageAddress::from(ppa))
            self.mm.read_page(&PhysicalPageAddress::from(ppa))
        else {
            return Err(FTL_ERR::READ_PAGE);
        };

        Ok(content)
    }

    pub fn write_page(&mut self, lpa: LogicalPageAddress) -> Result<(), FTL_ERR> {
        // Handle metadata in the FTL
        // Get a ppa from the provisoner (provisioners free list are guaranteed to have no bad blocks)
        let Ok(ppa) = self.provisioner.provison_page() else {
            return Err(FTL_ERR::WRITE_PAGE);
        };
        // Map the logical address we want to write to the physical address from the provisioner
        let Ok(()) = self.l2p_map.set_address_pairs(lpa, ppa.into()) else {
            return Err(FTL_ERR::WRITE_PAGE);
        };

        // Write the actual data with the media manager
        let Ok(()) = self.mm.write_page(&ppa) else {
            return Err(FTL_ERR::WRITE_PAGE);
        };
        Ok(())
    }
}

type PageContent = [u8; config::BYTES_PER_PAGE];

pub enum FTL_ERR {
    WRITE_PAGE,
    READ_PAGE,
}
