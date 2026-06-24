#![no_std]
#![no_main]

use user_lib::getpid;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("pid {}: Hello world from user mode program!", getpid());
    0
}
