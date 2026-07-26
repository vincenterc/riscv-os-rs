#![no_std]
#![no_main]

use alloc::vec::Vec;
use user_lib::{exit, thread_create, waittid};

#[macro_use]
extern crate user_lib;
extern crate alloc;

const THREAD_NUM: usize = 3;

fn thread_fn() {
    for _ in 0..300 {
        print!("a");
    }
    for _ in 0..300 {
        print!("b");
    }
    for _ in 0..300 {
        print!("c");
    }
    exit(0)
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut v: Vec<isize> = Vec::new();
    for _ in 0..THREAD_NUM {
        v.push(thread_create(linker_symbol_addr!(thread_fn), 0));
    }
    for tid in v.into_iter() {
        waittid(tid as usize);
    }
    println!("\nOK!");
    0
}
