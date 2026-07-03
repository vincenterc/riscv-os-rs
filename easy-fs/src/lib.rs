#![no_std]

extern crate alloc;

pub use block_dev::BlockDevice;

mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod layout;

pub const BLOCK_SZ: usize = 512;
