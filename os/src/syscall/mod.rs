use fs::sys_write;

use crate::syscall::process::{sys_exit, sys_fork, sys_get_time, sys_yield};

mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_FORK: usize = 220;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_FORK => sys_fork(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
