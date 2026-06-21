pub use context::TaskContext;

mod context;
mod manager;
mod pid;
mod switch;
mod task;

pub fn suspend_current_and_run_next() {
    todo!();
}

pub fn exit_current_and_run_next() {
    todo!();
}
