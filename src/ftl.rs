use crate::bad_block_table::table::{BadBlockTable, BlockStatus};
use crate::config;
use crate::core::address::{LogicalPageAddress, PhysicalPageAddress};
use crate::gc::gc::GarbageCollector;
use crate::logical_physical_address::mapper::L2pMapper;
use crate::media_manager::operations::{MediaManagerError, MediaOperations};
use crate::media_manager::stub::MEDIA_MANAGER;
use crate::provisioner::provisioner::{Block, Provisoner};

pub struct FTL {
    pub l2p_map: L2pMapper,
    pub provisioner: Provisoner,
    pub bbt: BadBlockTable,
    pub gc: GarbageCollector,
}

impl FTL {
    pub const fn new() -> Self {
        FTL {
            l2p_map: L2pMapper::new(),
            provisioner: Provisoner::new(),
            bbt: BadBlockTable::new(),
            gc: GarbageCollector::new(),
        }
    }

    pub fn init(&mut self) -> () {
        // Factory init bbt
        let _ = self.bbt.factory_init();

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

    pub fn read_page(&self, lpa: LogicalPageAddress) -> Result<PageContent, FtlErr> {
        let Some(ppa) = self.l2p_map.get_physical_address(lpa) else {
            return Err(FtlErr::ReadPage);
        };

        let Ok(content): Result<PageContent, MediaManagerError> =
            // self.mm.read_page(&PhysicalPageAddress::from(ppa))
            MEDIA_MANAGER.read_page(&PhysicalPageAddress::from(ppa))
        else {
            return Err(FtlErr::ReadPage);
        };

        Ok(content)
    }

    pub fn write_page(&mut self, lpa: LogicalPageAddress) -> Result<(), FtlErr> {
        // Handle metadata in the FTL
        // Get a ppa from the provisoner (provisioners free list are guaranteed to have no bad blocks)
        let Ok(ppa) = self.provisioner.provison_page() else {
            return Err(FtlErr::WritePage);
        };
        // Map the logical address we want to write to the physical address from the provisioner
        let Ok(()) = self.l2p_map.set_address_pairs(lpa, ppa.into()) else {
            return Err(FtlErr::WritePage);
        };

        // Write the actual data with the media manager
        let Ok(()) = MEDIA_MANAGER.write_page(&ppa) else {
            return Err(FtlErr::WritePage);
        };
        Ok(())
    }
}

type PageContent = [u8; config::BYTES_PER_PAGE];

pub enum FtlErr {
    WritePage,
    ReadPage,
}
