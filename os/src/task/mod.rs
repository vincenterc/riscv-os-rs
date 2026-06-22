use alloc::sync::Arc;
use lazy_static::lazy_static;

pub use context::TaskContext;
pub use processor::{current_trap_cx, current_user_token, run_tasks};

use crate::{
    loader::get_app_data_by_name,
    task::{
        manager::add_task,
        processor::{schedule, take_current_task},
        task::{TaskControlBlock, TaskStatus},
    },
};

mod context;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- stop exclusively accessing current TCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
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
