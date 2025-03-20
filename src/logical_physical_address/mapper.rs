use crate::config::TOTAL_PAGES;
use crate::core::address::{CompactPhysicalPageAddress, LogicalPageAddress};

pub struct L2pMapper {
    physical_to_logical: [Option<LogicalPageAddress>; TOTAL_PAGES],
    logical_to_physical: [Option<CompactPhysicalPageAddress>; TOTAL_PAGES],
}

//refactor to get / set only, decouple
impl L2pMapper {
    pub const fn new() -> Self {
        L2pMapper {
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
        lpa: LogicalPageAddress,
        cppa: CompactPhysicalPageAddress,
    ) -> Result<(), MappingError> {
        if lpa > self.logical_to_physical.len() - 1 {
            Err(MappingError::LogicalPageOutOfBounds)
        } else if cppa > self.physical_to_logical.len() - 1 {
            Err(MappingError::PhysicalPageOutOfBounds)
        } else {
            self.logical_to_physical[lpa] = Some(cppa);
            self.physical_to_logical[cppa] = Some(lpa);
            Ok(())
        }
    }

    pub fn get_physical_address(
        &self,
        lpa: LogicalPageAddress,
    ) -> Option<CompactPhysicalPageAddress> {
        self.logical_to_physical[lpa]
    }

    pub fn get_size(&self) -> Result<usize, MappingError> {
        if self.logical_to_physical.len() != self.physical_to_logical.len() {
            return Err(MappingError::SizeError);
        } else {
            return Ok(self.logical_to_physical.len());
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MappingError {
    ProvisionError,
    LogicalPageOutOfBounds,
    PhysicalPageOutOfBounds,
    SizeError
}
