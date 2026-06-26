#![no_std]
#![no_main]

use user_lib::{get_time, sleep};

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("into sleep test!");
    let start = get_time();
    println!("current time_msec = {}", start);
    sleep(100);
    let end = get_time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    println!("r_sleep passed!");
    0
}
