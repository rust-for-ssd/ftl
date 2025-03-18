use crate::config::{PAGES_PER_BLOCK, TOTAL_BLOCKS, TOTAL_PAGES};
use crate::core::address::{
    CompactPhysicalPageAddress, LogicalPageAddress, PhysicalBlockAddress, PhysicalPageAddress,
};
use crate::page_provisioner::provisioner::GlobalProvisoner;

pub struct L2P_Mapper {
    physical_to_logical: [Option<LogicalPageAddress>; TOTAL_PAGES],
    logical_to_physical: [Option<CompactPhysicalPageAddress>; TOTAL_PAGES],
}

//refactor to get / set only, decouple
impl L2P_Mapper {
    pub fn new() -> Self {
        L2P_Mapper {
            physical_to_logical: [None; TOTAL_PAGES],
            logical_to_physical: [None; TOTAL_PAGES],
        }
    }

    pub fn get_logical_address(
        &self,
        cppa: CompactPhysicalPageAddress,
    ) -> Option<LogicalPageAddress> {
        self.physical_to_logical[cppa]
    }

    pub fn set_address_pairs(
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

    pub fn get_physical_address(
        &self,
        lpa: LogicalPageAddress,
    ) -> Option<CompactPhysicalPageAddress> {
        self.logical_to_physical[lpa]
    }
}

enum MappingError {
    ProvisionError,
    LogicalPageOutOfBounds,
    PhysicalPageOutOfBounds,
}
