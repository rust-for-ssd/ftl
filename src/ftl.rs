use core::marker::PhantomData;

use crate::bad_block_table::table::{BadBlockTable, BlockStatus};
use crate::config;
use crate::core::address::{LogicalPageAddress, PhysicalPageAddress};
use crate::gc::gc::GarbageCollector;
use crate::logical_physical_address::mapper::L2pMapper;
use crate::media_manager::operations::{MediaManager, MediaManagerError};
use crate::media_manager::stub::MediaManagerStub;
use crate::provisioner::provisioner::{Block, Provisioner};

pub struct FTL<MM: MediaManager> {
    pub l2p_map: L2pMapper,
    pub provisioner: Provisioner,
    pub bbt: BadBlockTable,
    pub gc: GarbageCollector,
    phanthom_data: PhantomData<MM>,
}

pub static GLOBAL_FTL: FTL<MediaManagerStub> = FTL::<MediaManagerStub>::new();

impl<MM: MediaManager> FTL<MM> {
    pub const fn new() -> Self {
        FTL::<MM> {
            l2p_map: L2pMapper::new(),
            provisioner: Provisioner::new(),
            bbt: BadBlockTable::new(),
            gc: GarbageCollector::new(),
            phanthom_data: PhantomData::<MM>,
        }
    }

    pub fn init(&mut self) -> Result<(), FtlErr> {
        // Factory init bbt
        let Ok(()) = self.bbt.factory_init::<MM>() else {
            return Err(FtlErr::Init("Bad block table factory init error!"));
        };

        // Add good blocks from bbt to free list in provisioner
        for (channel_idx, ch) in self.bbt.channel_bad_block_tables.iter().enumerate() {
            for (lun_idx, lun) in ch.luns.iter().enumerate() {
                for (plane_idx, plane) in lun.planes.iter().enumerate() {
                    for (block_idx, block_status) in plane.blocks.iter().enumerate() {
                        if *block_status == BlockStatus::Good {
                            // TODO make this a method in the provisoner and set fields to private
                            let Ok(()) = self.provisioner.channel_provisioners[channel_idx].luns
                                [lun_idx]
                                .free
                                .push(Block {
                                    id: block_idx,
                                    plane_id: plane_idx,
                                })
                            else {
                                return Err(FtlErr::Init(
                                    "Could not push to free list in factory init",
                                ));
                            };
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
            MM::read_page(&PhysicalPageAddress::from(ppa))
        else {
            return Err(FtlErr::ReadPage("Media manager error!"));
        };

        Ok(content)
    }

    pub fn write_page(&mut self, lpa: LogicalPageAddress) -> Result<(), FtlErr> {
        // Handle metadata in the FTL
        // Get a ppa from the provisoner (provisioners free list are guaranteed to have no bad blocks)
        let Ok(ppa) = self.provisioner.provision_page() else {
            return Err(FtlErr::WritePage("Provision error!"));
        };
        // Map the logical address we want to write to the physical address from the provisioner
        let Ok(()) = self.l2p_map.set_address_pairs(lpa, ppa.into()) else {
            return Err(FtlErr::WritePage("Mapping error"));
        };

        // Write the actual data with the media manager
        let Ok(()) = MM::write_page(&ppa) else {
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
