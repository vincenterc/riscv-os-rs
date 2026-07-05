pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr};
pub use frame_allocator::{FrameTracker, frame_alloc, frame_dealloc};
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet, kernel_token, remap_test};
pub use page_table::{
    PageTable, UserBuffer, translated_byte_buffer, translated_refmut, translated_str,
};

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
