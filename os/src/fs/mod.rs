use crate::mm::UserBuffer;

pub use inode::{OpenFlags, list_apps, open_file};
pub use stdio::{Stdin, Stdout};

mod inode;
mod stdio;

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}
