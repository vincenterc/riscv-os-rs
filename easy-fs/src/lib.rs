#![no_std]

extern crate alloc;

pub use block_dev::BlockDevice;

mod block_cache;
mod block_dev;

pub const BLOCK_SZ: usize = 512;
