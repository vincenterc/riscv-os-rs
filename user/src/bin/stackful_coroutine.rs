#![no_std]
#![no_main]

use alloc::{vec, vec::Vec};
use user_lib::exit;

#[macro_use]
extern crate user_lib;
extern crate alloc;

const DEFAULT_STACK_SIZE: usize = 1024; // 256 got SEGFAULT, 1024 got right results, 4096 got failed memory allocation.
const MAX_TASKS: usize = 5;
static mut RUNTIME: usize = 0;

core::arch::global_asm!(include_str!("stackful_coroutine_switch.S"));

unsafe extern "C" {
    fn switch(old: *mut TaskContext, new: *const TaskContext);
}

pub struct Runtime {
    tasks: Vec<Task>,
    current: usize,
}

impl Runtime {
    pub fn new() -> Self {
        // This will be our base task, which will be initialized in the `running` state
        let base_task = Task {
            id: 0,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: TaskContext::default(),
            state: State::Running,
        };

        // We initialize the rest of our tasks.
        let mut tasks = vec![base_task];
        let mut available_tasks: Vec<Task> = (1..MAX_TASKS).map(|i| Task::new(i)).collect();
        tasks.append(&mut available_tasks);

        Runtime { tasks, current: 0 }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .tasks
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available task.");

        println!("RUNTIME: spawning task {}", available.id);
        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !7) as *mut u8;

            available.ctx.x1 = linker_symbol_addr!(guard) as u64; // ctx.x1  is old return address
            available.ctx.nx1 = linker_symbol_addr!(f) as u64; // ctx.nx1 is new return address
            available.ctx.x2 = s_ptr.offset(-32) as u64; // cxt.x2 is sp
        }
        available.state = State::Ready;
    }

    pub fn run(&mut self) {
        while self.t_yield() {}
        println!("All tasks finished!");
    }

    fn t_return(&mut self) {
        if self.current != 0 {
            self.tasks[self.current].state = State::Available;
            self.t_yield();
        }
    }

    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.tasks[pos].state != State::Ready {
            pos += 1;
            if pos == self.tasks.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }

        if self.tasks[self.current].state != State::Available {
            self.tasks[self.current].state = State::Ready;
        }

        self.tasks[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            switch(&mut self.tasks[old_pos].ctx, &self.tasks[pos].ctx);
        }

        self.tasks.len() > 0
    }
}

#[derive(PartialEq)]
enum State {
    Available,
    Running,
    Ready,
}

struct Task {
    id: usize,
    stack: Vec<u8>,
    ctx: TaskContext,
    state: State,
}

impl Task {
    fn new(id: usize) -> Self {
        Task {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: TaskContext::default(),
            state: State::Available,
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct TaskContext {
    // 15 u64
    x1: u64,  //ra: return address
    x2: u64,  //sp
    x8: u64,  //s0,fp
    x9: u64,  //s1
    x18: u64, //x18-27: s2-11
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    nx1: u64, //new return address
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    };
}

pub fn yield_task() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}

#[unsafe(no_mangle)]
pub fn main() {
    println!("stackful_coroutine begin...");
    println!("TASK  0(Runtime) STARTING");
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("TASK  1 STARTING");
        let id = 1;
        for i in 0..4 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("TASK 1 FINISHED");
    });
    runtime.spawn(|| {
        println!("TASK 2 STARTING");
        let id = 2;
        for i in 0..8 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("TASK 2 FINISHED");
    });
    runtime.spawn(|| {
        println!("TASK 3 STARTING");
        let id = 3;
        for i in 0..12 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("TASK 3 FINISHED");
    });
    runtime.spawn(|| {
        println!("TASK 4 STARTING");
        let id = 4;
        for i in 0..16 {
            println!("task: {} counter: {}", id, i);
            yield_task();
        }
        println!("TASK 4 FINISHED");
    });
    runtime.run();
    println!("stackful_coroutine PASSED");
    exit(0);
}
