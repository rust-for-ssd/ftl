use core::marker::PhantomData;

use crate::bad_block_table::table::{BadBlockTable, BlockStatus};
use crate::config;
use crate::core::address::{LogicalPageAddress, PhysicalPageAddress};
use crate::gc::gc::GarbageCollector;
use crate::logical_physical_address::mapper::L2pMapper;
use crate::media_manager::operations::{MediaManagerError, MediaOperations};
use crate::media_manager::stub::MediaManager;
use crate::provisioner::provisioner::{Block, Provisoner};

pub struct FTL<MO: MediaOperations> {
    pub l2p_map: L2pMapper,
    pub provisioner: Provisoner,
    pub bbt: BadBlockTable,
    pub gc: GarbageCollector,
    phanthom_data: PhantomData<MO>,
}

pub static GLOBAL_FTL: FTL<MediaManager> = FTL::<MediaManager>::new();

impl<MO: MediaOperations> FTL<MO> {
    pub const fn new() -> Self {
        FTL::<MO> {
            l2p_map: L2pMapper::new(),
            provisioner: Provisoner::new(),
            bbt: BadBlockTable::new(),
            gc: GarbageCollector::new(),
            phanthom_data: PhantomData::<MO>,
        }
    }

    pub fn init(&mut self) -> Result<(), FtlErr> {
        // Factory init bbt
        let Ok(()) = self.bbt.factory_init::<MO>() else {
            return Err(FtlErr::Init("Bad block table factory init error!"));
        };

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
        Ok(())
    }

    pub fn read_page(&self, lpa: LogicalPageAddress) -> Result<PageContent, FtlErr> {
        let Some(ppa) = self.l2p_map.get_physical_address(lpa) else {
            return Err(FtlErr::ReadPage("Mapping error!"));
        };

        let Ok(content): Result<PageContent, MediaManagerError> =
            MO::read_page(&PhysicalPageAddress::from(ppa))
        else {
            return Err(FtlErr::ReadPage("Media manager error!"));
        };

        Ok(content)
    }

    pub fn write_page(&mut self, lpa: LogicalPageAddress) -> Result<(), FtlErr> {
        // Handle metadata in the FTL
        // Get a ppa from the provisoner (provisioners free list are guaranteed to have no bad blocks)
        let Ok(ppa) = self.provisioner.provison_page() else {
            return Err(FtlErr::WritePage("Provision error!"));
        };
        // Map the logical address we want to write to the physical address from the provisioner
        let Ok(()) = self.l2p_map.set_address_pairs(lpa, ppa.into()) else {
            return Err(FtlErr::WritePage("Mapping error"));
        };

        // Write the actual data with the media manager
        let Ok(()) = MO::write_page(&ppa) else {
            return Err(FtlErr::WritePage("Media manager error"));
        };
        Ok(())
    }
}

type PageContent = [u8; config::BYTES_PER_PAGE];

pub enum FtlErr<'a> {
    Init(&'a str),
    WritePage(&'a str),
    ReadPage(&'a str),
}
