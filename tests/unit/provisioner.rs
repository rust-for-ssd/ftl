use core::mem::transmute_copy;

use ftl::{
    bad_block_table::table::{BlockStatus, ChannelBadBlockTable},
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
    ftl::FTL,
    media_manager::operations::{MediaManager, MediaManagerError},
    provisioner::provisioner::{Block, BlockWithPageInfo, Page, Provisioner},
};

#[test_case]
pub fn new() {
    let prov = Provisioner::new();
    assert_eq!(prov.channel_provisioners.len(), config::N_CHANNELS)
}

#[test_case]
pub fn provision_block() {
    let mut prov = Provisioner::new();

    // No free blocks when creating new
    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );

    let block = Block { id: 3, plane_id: 0 };
    let res = prov.channel_provisioners[0].luns[0].free.push(block);
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 1);

    let res = prov.provision_block();
    assert_eq!(
        res,
        Ok(PhysicalBlockAddress {
            channel_id: 0,
            lun_id: 0,
            plane_id: 0,
            block_id: 3
        })
    );

    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 0);

    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );
}

#[test_case]
pub fn provision_page() {
    let mut prov = Provisioner::new();

    // No free blocks, meaning no free pages when creating new
    let res = prov.provision_page();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreePage)
    );

    let block = Block { id: 3, plane_id: 0 };
    let res = prov.channel_provisioners[0].luns[0].free.push(block);
    assert!(res.is_ok());
    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 1);

    let res = prov.provision_page();
    assert_eq!(
        res,
        Ok(PhysicalPageAddress {
            channel_id: 0,
            lun_id: 0,
            plane_id: 0,
            block_id: 3,
            page_id: 0
        })
    );
    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 0);
    let size = prov.channel_provisioners[0].luns[0]
        .partially_used
        .get_size();
    assert_eq!(size, 1);
    let size = prov.channel_provisioners[0].luns[0].used.get_size();
    assert_eq!(size, 0);

    for i in 1..config::PAGES_PER_BLOCK {
        let size = prov.channel_provisioners[0].luns[0]
            .partially_used
            .get_size();
        assert_eq!(size, 1);
        let res = prov.provision_page();
        assert_eq!(
            res,
            Ok(PhysicalPageAddress {
                channel_id: 0,
                lun_id: 0,
                plane_id: 0,
                block_id: 3,
                page_id: i
            })
        );
    }
    let size = prov.channel_provisioners[0].luns[0]
        .partially_used
        .get_size();
    assert_eq!(size, 0);
    let size = prov.channel_provisioners[0].luns[0].used.get_size();
    assert_eq!(size, 1);
    let res = prov.provision_page();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreePage)
    );
}

#[test_case]
pub fn provision_page_with_partially_used_blocks() {
    let mut prov = Provisioner::new();

    // No free blocks, meaning no free pages when creating new
    let res = prov.provision_page();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreePage)
    );

    let block = BlockWithPageInfo {
        id: 3,
        plane_id: 0,
        pages: [Page::Free; config::PAGES_PER_BLOCK],
    };
    let res = prov.channel_provisioners[0].luns[0]
        .partially_used
        .push(block);
    assert!(res.is_ok());
    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 0);
    let size = prov.channel_provisioners[0].luns[0]
        .partially_used
        .get_size();
    assert_eq!(size, 1);

    let res = prov.provision_page();
    assert_eq!(
        res,
        Ok(PhysicalPageAddress {
            channel_id: 0,
            lun_id: 0,
            plane_id: 0,
            block_id: 3,
            page_id: 0
        })
    );
    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 0);
    let size = prov.channel_provisioners[0].luns[0]
        .partially_used
        .get_size();
    assert_eq!(size, 1);
    let size = prov.channel_provisioners[0].luns[0].used.get_size();
    assert_eq!(size, 0);

    for i in 1..config::PAGES_PER_BLOCK {
        let size = prov.channel_provisioners[0].luns[0]
            .partially_used
            .get_size();
        assert_eq!(size, 1);
        let res = prov.provision_page();
        assert_eq!(
            res,
            Ok(PhysicalPageAddress {
                channel_id: 0,
                lun_id: 0,
                plane_id: 0,
                block_id: 3,
                page_id: i
            })
        );
    }
    let size = prov.channel_provisioners[0].luns[0]
        .partially_used
        .get_size();
    assert_eq!(size, 0);
    let size = prov.channel_provisioners[0].luns[0].used.get_size();
    assert_eq!(size, 1);
    let res = prov.provision_page();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreePage)
    );
}

#[test_case]
pub fn provision_block_from_different_channels() {
    let mut prov = Provisioner::new();

    // No free blocks when creating new
    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );

    let block = Block { id: 3, plane_id: 0 };
    let res = prov.channel_provisioners[0].luns[0].free.push(block);
    assert!(res.is_ok());
    let res = prov.channel_provisioners[2].luns[3].free.push(block);
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 1);

    let res = prov.provision_block();
    assert!(res.is_ok());

    let res = prov.provision_block();
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[0].free.get_size();
    assert_eq!(size, 0);

    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );
}

#[test_case]
pub fn push_free_block() {
    let mut prov = Provisioner::new();

    // No free blocks when creating new
    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );

    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    let res = prov.push_free_block(&pba);
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[2].free.get_size();
    assert_eq!(size, 1);

    let res = prov.provision_block();
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[2].free.get_size();
    assert_eq!(size, 0);

    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );
}

#[test_case]
pub fn multiple_push_free_block() {
    let mut prov = Provisioner::new();

    // No free blocks when creating new
    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );

    let pba = PhysicalBlockAddress {
        channel_id: 0,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    let res = prov.push_free_block(&pba);
    assert!(res.is_ok());

    let size = prov.channel_provisioners[0].luns[2].free.get_size();
    assert_eq!(size, 1);

    let pba = PhysicalBlockAddress {
        channel_id: 2,
        lun_id: 2,
        plane_id: 0,
        block_id: 3,
    };
    let res = prov.push_free_block(&pba);
    assert!(res.is_ok());

    let size = prov.channel_provisioners[2].luns[2].free.get_size();
    assert_eq!(size, 1);

    let res = prov.provision_block();
    assert!(res.is_ok());

    let res = prov.provision_block();
    assert!(res.is_ok());

    let res = prov.provision_block();
    assert_eq!(
        res,
        Err(ftl::provisioner::provisioner::ProvisionError::NoFreeBlock)
    );
}

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

#[test_case]
pub fn ftl_factory_init_then_provision() {
    let mut ftl = FTL::<OkMediaManager>::new();
    assert!(ftl.provisioner.provision_block().is_err());
    assert!(ftl.provisioner.provision_page().is_err());
    assert!(ftl.init().is_ok());
    assert!(ftl.provisioner.provision_block().is_ok());
    assert!(ftl.provisioner.provision_page().is_ok());
}
