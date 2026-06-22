use alloc::sync::Arc;
use lazy_static::lazy_static;

pub use context::TaskContext;
pub use processor::{current_trap_cx, current_user_token, run_tasks};

use crate::{
    loader::get_app_data_by_name,
    task::{manager::add_task, task::TaskControlBlock},
};

mod context;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

pub fn suspend_current_and_run_next() {
    todo!();
}

pub fn exit_current_and_run_next() {
    todo!();
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}
