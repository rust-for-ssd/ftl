use crate::config::{PAGES_PER_BLOCK, TOTAL_BLOCKS, TOTAL_PAGES};
use crate::core::address::{CompactPhysicalPageAddress, LogicalPageAddress, PhysicalBlockAddress, PhysicalPageAddress};
use crate::page_provisioner::provisioner::GlobalProvisoner;

// struct L2P_Mapping {
//     blocks: BlockMapping,
//     pages: PageMapping,
// }

// struct BlockMapping {
//     logical_blocks: [Block; TOTAL_BLOCKS],
//     physical_blocks: [Block; TOTAL_BLOCKS],
// }

// struct Block {
//     pages: [Page; PAGES_PER_BLOCK],
// }


struct PageMapping {
    physical_to_logical: [Option<LogicalPageAddress>; TOTAL_PAGES],
    logical_to_physical: [Option<CompactPhysicalPageAddress>; TOTAL_PAGES],
}

//refactor to get / set only, decouple
impl PageMapping {
    fn new(provisioner: &GlobalProvisoner) -> Self {
        PageMapping {
            physical_to_logical: [None; TOTAL_PAGES],
            logical_to_physical: [None; TOTAL_PAGES],
        }
    }

    fn get_logical_address(&self, cppa: CompactPhysicalPageAddress) -> Option<LogicalPageAddress> {
        self.physical_to_logical[cppa]
    }

    fn set_address_pairs(
        &mut self,
        lpa_idx: LogicalPageAddress,
        cppa: CompactPhysicalPageAddress,
    ) -> Result<(), MappingError> {
        if lpa_idx > self.logical_to_physical.len() - 1 {
            Err(MappingError::LogicalPageOutOfBounds)
        } else if cppa > self.physical_to_logical.len() - 1 {
            Err(MappingError::PhysicalPageOutOfBounds)
        } else {
            self.logical_to_physical[lpa_idx] = Some(cppa);
            self.physical_to_logical[cppa] = Some(lpa_idx);
            Ok(())
        }
    }

    fn get_physical_address(&self, lpa: LogicalPageAddress) -> Option<CompactPhysicalPageAddress> {
        self.logical_to_physical[lpa]
    }

    // fn logical_to_physical_page_address(
    //     &self,
    //     lpa: LogicalPageAddress,
    // ) -> Result<PhysicalPageAddress, MappingError> {
    //     if lpa > TOTAL_PAGES {
    //         return Err(MappingError::LogicalPageOutOfBounds);
    //     }

    //     if let Some(page){
    //         self.physical_pages[lpa] = page;
    //         Ok ()
    //     } else {
    //         match self.provisoner.provison_page() {
    //             Ok(ppa) => {
    //                 self.logical_pages[lpa] = Some(ppa.into());

    //                 Ok(ppa)
    //             }
    //             Err(_) => Err(MappingError::ProvisionError),
    //         }
    //     }
    // }

    // fn physical_to_logical_page_address(&self, pba: PhysicalPageAddress) -> LogicalPageAddress {
    //     let id =
    //     self.logical_pages[]

    // }
}

enum MappingError {
    ProvisionError,
    LogicalPageOutOfBounds,
    PhysicalPageOutOfBounds,
}
