use crate::config::{TOTAL_BLOCKS, TOTAL_PAGES, PAGES_PER_BLOCK};
use crate::core::address::{PhysicalBlockAddress, PhysicalPageAddress, CompactPhysicalPageAddress};
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


type LogicalPageAddress = usize; //range [0, config::N_pages]

struct PageMapping {
    provisoner: &GlobalProvisoner,
    logical_pages: [Option<LogicalPageAddress>; TOTAL_PAGES],
    physical_pages: [Option<CompactPhysicalPageAddress>; TOTAL_PAGES],
}

impl PageMapping {
    fn new(provisioner: &GlobalProvisoner) -> Self {
        PageMapping {
            provisoner: provisioner,
            logical_pages: [None; TOTAL_PAGES],
            physical_pages: [None; TOTAL_PAGES],
        }
    }

    fn logical_to_physical_page_address(
        &self,
        lpa: LogicalPageAddress,
    ) -> Result<PhysicalPageAddress, MappingError> {
        if lpa > TOTAL_PAGES {
            return Err(MappingError::LogicalPageOutOfBounds);
        }

        if let Some(page) = self.physical_pages[lpa] {
            page
        } else {
            match self.provisoner.provison_page() {
                Ok(ppa) => {
                    self.logical_pages[lpa] = Some(ppa.into());

                    Ok(ppa)
                }
                Err(_) => Err(MappingError::ProvisionError),
            }
        }
    }

    // fn physical_to_logical_page_address(&self, pba: PhysicalPageAddress) -> LogicalPageAddress {
    //     let id =
    //     self.logical_pages[]

    // }
}

enum MappingError {
    ProvisionError,
    LogicalPageOutOfBounds,
}
