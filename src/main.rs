#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[repr(C, packed)]
pub struct Multiboot2Header {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_tag: u16,
    end_flags: u16,
    end_size: u32,
}

const MAGIC: u32 = 0xE85250D6;
const ARCH: u32 = 0; // x86
const LENGTH: u32 = 24;

#[link_section = ".multiboot_header"]
#[no_mangle]
pub static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    magic: MAGIC,
    architecture: ARCH,
    header_length: LENGTH,
    checksum: (0u32.wrapping_sub(MAGIC).wrapping_sub(ARCH).wrapping_sub(LENGTH)),
    end_tag: 0,
    end_flags: 0,
    end_size: 8,
};

fn serial_write(s: &str) {
    for b in s.bytes() {
        unsafe {
            asm!("out dx, al", in("dx") 0x3f8u16, in("al") b);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_write("\r\n==========================================\r\n");
    serial_write("  Silvae Bare-Metal Hypervisor Booted!    \r\n");
    serial_write("  True Frontier (2f837bf7) Initialized.   \r\n");
    serial_write("==========================================\r\n");
    
    let vga_buffer = 0xb8000 as *mut u8;
    let hello = b"Silvae Bare-Metal Hypervisor Booted!";
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
        }
    }
    
    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
