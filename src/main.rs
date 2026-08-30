#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::arch::x86_64::_rdtsc;

core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

extern "C" {
    static header_start: u32;
}
#[used]
static KEEP_HEADER: &u32 = unsafe { &header_start };

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    
    // Generate a pseudo-random 8-digit PIN using the CPU's Time Stamp Counter
    let tsc = unsafe { _rdtsc() };
    // Mix the bits using a simple Linear Congruential Generator so it looks nicely random
    let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
    let mut pin = ((lcg >> 32) % 100000000) as u32;
    
    let mut pin_str = [b'0'; 9];
    for i in (0..9).rev() {
        if i == 4 {
            pin_str[i] = b' ';
        } else {
            pin_str[i] = b'0' + (pin % 10) as u8;
            pin /= 10;
        }
    }
    
    let mut lines = [
        *b"================================================================================",
        *b"                                                                                ",
        *b"                               Welcome to Silvae!                               ",
        *b"                                                                                ",
        *b"================================================================================",
        *b"                                                                                ",
        *b"  Connected to $NETWORKID                                                       ",
        *b"                                                                                ",
        *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
        *b"  http://Silv.ae/start:                                                         ",
        *b"                                                                                ",
        *b"                                   1234 5678                                    ",
        *b"                                                                                ",
    ];
    
    // Inject the randomly generated PIN directly into the 11th line (index 11)
    lines[11][35..44].copy_from_slice(&pin_str);

    for (row, line) in lines.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            unsafe {
                *vga_buffer.offset((row * 80 + col) as isize * 2) = byte;
                *vga_buffer.offset((row * 80 + col) as isize * 2 + 1) = 0x0a; // Light Green text
            }
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
