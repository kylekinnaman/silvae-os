/// Kernel heap allocation mappings required for dynamic buffer generation.
#[cfg(not(test))]
use linked_list_allocator::LockedHeap;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Virtual start address allocated for the kernel heap.
pub const HEAP_START: usize = 0x4444_4444_0000;
/// Size of the static heap buffer in bytes (256 KiB).
pub const HEAP_SIZE: usize = 256 * 1024;

/// Returns the configured size of the kernel heap in bytes.
pub const fn heap_size() -> usize {
    HEAP_SIZE
}

/// Returns the configured starting virtual address of the kernel heap.
pub const fn heap_start() -> usize {
    HEAP_START
}

/// Returns the configured ending virtual address of the kernel heap.
pub const fn heap_end() -> usize {
    HEAP_START + HEAP_SIZE
}

/// Maps a predefined physical memory block into the kernel virtual heap space.
#[cfg(not(test))]
pub fn init_heap() {
    static mut HEAP_BUFFER: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        let ptr = core::ptr::addr_of_mut!(HEAP_BUFFER) as *mut u8;
        ALLOCATOR.lock().init(ptr, HEAP_SIZE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_constants() {
        assert_eq!(heap_size(), 256 * 1024);
        assert_eq!(heap_start(), 0x4444_4444_0000);
        assert_eq!(heap_end(), 0x4444_4444_0000 + 256 * 1024);
    }

    #[test]
    fn test_heap_bounds_validity() {
        assert!(heap_end() > heap_start());
        assert_eq!(heap_end() - heap_start(), heap_size());
    }
}
