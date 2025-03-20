use ftl::config::TOTAL_PAGES;
use ftl::core::address::{CompactPhysicalPageAddress, PhysicalPageAddress};
use ftl::logical_physical_address::mapper::L2pMapper;
use ftl::logical_physical_address::mapper::MappingError;
use semihosting::println;

#[test_case]
pub fn initializes_new_mapper_of_crrect_size() {
    let mapper = L2pMapper::new();
    let result = mapper.get_size();
    assert_eq!(result, Ok(TOTAL_PAGES));
}

#[test_case]
pub fn physical_page_address_to_compact_physical_page_address() {
    let ppa = PhysicalPageAddress {
        channel_id: 1,
        lun_id: 1,
        plane_id: 0,
        block_id: 1,
        page_id: 1,
    };
    let cppa: CompactPhysicalPageAddress = ppa.into();

    // assert_eq!(cppa,)
}

#[test_case]
pub fn compact_physical_page_address_to_physical_page_address() {
    let ppa = PhysicalPageAddress {
        channel_id: 1,
        lun_id: 1,
        plane_id: 0,
        block_id: 1,
        page_id: 1,
    };

    let cppa: CompactPhysicalPageAddress = ppa.into();
    println!("{:?}", cppa);
    let ppa_new: PhysicalPageAddress = cppa.into();

    assert_eq!(ppa, ppa_new);
}

#[test_case]
pub fn set_address_pairs() {
    let mut mapper = L2pMapper::new();
    let lpa = 42;
    let ppa = PhysicalPageAddress {
        channel_id: 1,
        lun_id: 1,
        plane_id: 0,
        block_id: 1,
        page_id: 1,
    };
    let cppa: CompactPhysicalPageAddress = ppa.into();

    let _ = mapper.set_address_pairs(lpa, cppa);

    let cppa_out = mapper.get_physical_address(lpa).unwrap();
    let lpa_out = mapper.get_logical_address(cppa).unwrap();

    assert_eq!(cppa, cppa_out);
    assert_eq!(lpa, lpa_out);
}

#[test_case]
pub fn set_invalid_logical_addr() {
    let mut mapper = L2pMapper::new();
    let lpa_too_large = TOTAL_PAGES + 1;
    let ppa = PhysicalPageAddress {
        channel_id: 1,
        lun_id: 1,
        plane_id: 0,
        block_id: 1,
        page_id: 1,
    };
    let cppa: CompactPhysicalPageAddress = ppa.into();

    let res = mapper.set_address_pairs(lpa_too_large, cppa);

    assert_eq!(res, Err(MappingError::LogicalPageOutOfBounds));
}

#[test_case]
pub fn set_invalid_physical_addr() {
    let mut mapper = L2pMapper::new();
    let lpa = 1;

    let wrong_cppa: CompactPhysicalPageAddress = usize::MAX;

    let res = mapper.set_address_pairs(lpa, wrong_cppa);

    assert_eq!(res, Err(MappingError::PhysicalPageOutOfBounds));
}
