use ftl::{
    config,
    core::address::{PhysicalBlockAddress, PhysicalPageAddress},
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
