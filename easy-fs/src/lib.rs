#![no_std]

extern crate alloc;

pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
pub use vfs::Inode;

mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod layout;
mod vfs;

pub const BLOCK_SZ: usize = 512;
