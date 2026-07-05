#![no_std]
#![no_main]

use user_lib::{OpenFlags, close, open, read};

extern crate alloc;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let fd = open("filea\0", OpenFlags::RDONLY);
    if fd == -1 {
        panic!("Error occurred when opening file");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 256];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        println!("{}", core::str::from_utf8(&buf[..size]).unwrap());
    }
    close(fd);
    0
}
