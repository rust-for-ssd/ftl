use ftl::bad_block_table::table::{BlockStatus, ChannelBadBlockTable};
use ftl::media_manager::stub::PhysicalBlockAddress;

#[test_case]
pub fn new_get_set_status() {
    let mut bbt = ChannelBadBlockTable::new(0);
    let pba = PhysicalBlockAddress {
        channel: 0,
        lun: 0,
        plane: 0,
        block: 0,
    };

    assert_eq!(bbt.get_block_status(&pba), BlockStatus::Good);

    bbt.set_block_status(&pba, BlockStatus::Bad);
    assert_eq!(bbt.get_block_status(&pba), BlockStatus::Bad);

    bbt.set_block_status(&pba, BlockStatus::Reserved);
    assert_eq!(bbt.get_block_status(&pba), BlockStatus::Reserved);
}
