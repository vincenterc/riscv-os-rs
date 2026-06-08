pub use heap_allocator::heap_test;

mod heap_allocator;

pub fn init() {
    heap_allocator::init_heap();
}
