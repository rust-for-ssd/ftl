#![no_std]
#![no_main]

mod bad_block_table;
mod media_manager;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct MyStruct {
    x: i32,
}
