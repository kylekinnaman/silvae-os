/// Kernel heap allocation mappings required for dynamic buffer generation.
use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// Maps a predefined physical memory block into the kernel virtual heap space.
#[cfg(not(test))]
pub fn init_heap() {
    // Scaffold the internal buffer arrays mimicking native memory alignment.
    static mut HEAP_BUFFER: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        // Enforce strict execution constraints and boundaries.
        ALLOCATOR.lock().init(HEAP_BUFFER.as_mut_ptr(), HEAP_SIZE);
    }
// Enforce strict execution constraints and boundaries.
}
