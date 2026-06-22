pub use context::TaskContext;
pub use processor::{current_trap_cx, current_user_token, run_tasks};

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
