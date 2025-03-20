use core::mem::transmute_copy;
use ftl::{
    bad_block_table::table::{
        BadBlockTable, BadBlockTableError, BlockStatus, ChannelBadBlockTable,
    },
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    media_manager::operations::{MediaManager, MediaManagerError},
};

pub struct OkMediaManager {}

impl MediaManager for OkMediaManager {
    fn erase_block(pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        match pba {
            PhysicalBlockAddress {
                channel_id: 0,
                lun_id: 0,
                plane_id: 0,
                block_id: 2,
            } => Err(MediaManagerError::Erase),
            _ => Ok(()),
        }
    }

    fn read_page<T>(ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        match ppa {
            PhysicalPageAddress {
                channel_id: ch_id,
                lun_id: 0,
                plane_id: 0,
                block_id: 0,
                page_id: 0,
            } => {
                let mut ch_bbt = ChannelBadBlockTable::new(*ch_id);
                ch_bbt.luns[0].planes[0].blocks[1] = BlockStatus::Bad;
                Ok(unsafe { transmute_copy::<_, T>(&ch_bbt) })
            }
            _ => {
                let page = [0; config::BYTES_PER_PAGE];
                Ok(unsafe { transmute_copy::<_, T>(&page) })
            }
        }
    }

    fn read_block<T>(_pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        todo!()
    }

    fn write_page(_ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Ok(())
    }
}

pub struct ErrMediaManager {}

impl MediaManager for ErrMediaManager {
    fn erase_block(_pba: &PhysicalBlockAddress) -> Result<(), MediaManagerError> {
        Err(MediaManagerError::Erase)
    }

    fn read_page<T>(_ppa: &PhysicalPageAddress) -> Result<T, MediaManagerError> {
        Err(MediaManagerError::Read)
    }

    fn read_block<T>(_pba: &PhysicalBlockAddress) -> Result<T, MediaManagerError> {
        Err(MediaManagerError::Read)
    }

    fn write_page(_ppa: &PhysicalPageAddress) -> Result<(), MediaManagerError> {
        Err(MediaManagerError::Write)
    }
}

#[test_case]
pub fn factory_init() {
    let mut bbt = BadBlockTable::new();
    let res = bbt.factory_init::<OkMediaManager>();
    assert_eq!(res, Ok(()));

    let ch_bbt = bbt.channel_bad_block_tables[0];
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 0,
        plane_id: 0,
        block_id: 2,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Bad);

    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Good);
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 0,
        plane_id: 0,
        block_id: 0,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Reserved);

    let res = bbt.factory_init::<ErrMediaManager>();
    assert_eq!(res, Err(BadBlockTableError::FactoryInitTable));
}

#[test_case]
pub fn block_status() {
    let bbt = BadBlockTable::new();
    let mut ch_bbt = bbt.channel_bad_block_tables[0];
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Good);

    ch_bbt.luns[2].planes[0].blocks[3] = BlockStatus::Bad;
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Bad);

    ch_bbt.luns[2].planes[0].blocks[3] = BlockStatus::Reserved;
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Reserved);
}

#[test_case]
pub fn bbt_block_reserved() {
    let bbt = BadBlockTable::new();
    let ch_bbt = bbt.channel_bad_block_tables[0];
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 0,
        plane_id: 0,
        block_id: 0,
    };

    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Reserved);
}

#[test_case]
pub fn restore_state_from_boot() {
    let res = BadBlockTable::restore_state_from_boot::<OkMediaManager>();
    assert!(res.is_ok());

    let ch_bbt = res.unwrap().channel_bad_block_tables[0];
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 0,
        plane_id: 0,
        block_id: 1,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Bad);

    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Good);
    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 0,
        plane_id: 0,
        block_id: 0,
    };
    assert_eq!(ch_bbt.get_block_status(&pba), BlockStatus::Reserved);

    let res = BadBlockTable::restore_state_from_boot::<ErrMediaManager>();
    assert_eq!(res, Err(BadBlockTableError::RestoreTable));
}
