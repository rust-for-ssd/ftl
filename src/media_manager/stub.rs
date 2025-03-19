use crate::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
};

use core::mem::transmute_copy;

use super::media_manager::MediaManager;
use super::media_manager::MediaManagerError;

pub struct MediaManagerStub;

const MMS: MediaManagerStub = MediaManagerStub {};

impl MediaManager for MediaManagerStub {
    fn erase_block(pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }

    fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        // We simulate
        let page = [0; config::BYTES_PER_PAGE];
        Ok(unsafe { transmute_copy::<_, T>(&page) })
    }

    fn write_page(ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}
