#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]

pub mod allocator;
pub mod apic;
pub mod dma;
pub mod interrupts;
pub mod net;
pub mod pairing;
pub mod pci;
pub mod port;
pub mod rtl8139;

extern crate alloc;

#[cfg(not(test))]
use core::arch::asm;
#[cfg(not(test))]
use core::arch::x86_64::_rdtsc;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

#[cfg(not(test))]
extern "C" {
    static header_start: u32;
}

#[cfg(not(test))]
#[used]
static KEEP_HEADER: &u32 = unsafe { &header_start }; // SAFETY: Linked statically from `boot.s` Multiboot header symbol.

/// Bare-metal kernel entry point invoked from `boot.s`.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    crate::allocator::init_heap();
    crate::interrupts::init_idt();

    // SAFETY: IDT is initialized and interrupts remain masked prior to `enable()`.
    unsafe { crate::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let devices = crate::pci::enumerate_pci();
    let network_adapters = crate::pci::find_network_adapters(&devices);
    for (dev, driver) in network_adapters {
        dev.enable_bus_mastering();

        match driver {
            crate::pci::NetworkDriver::Realtek8139 => {
                let io_base = (dev.read_bar(0) & !3) as u16;
                let nic = crate::rtl8139::Rtl8139::new(io_base);
                *crate::rtl8139::RTL8139_NIC.lock() = Some(nic);
                crate::net::init_network();
                break;
            }
            crate::pci::NetworkDriver::IntelE1000 | crate::pci::NetworkDriver::VirtIoNet => {}
        }
    }

    // SAFETY: Executing on x86_64 bare-metal hardware where RDTSC is always present.
    let tsc = unsafe { _rdtsc() };
    let pin = crate::pairing::derive_pairing_pin(tsc);
    let pin_str = crate::pairing::format_pairing_pin(pin);
    let lines = crate::pairing::build_welcome_banner(&pin_str);

    // SAFETY: COM1 I/O port base `0x3F8` is standard x86 serial hardware.
    let mut serial_port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    serial_port.init();
    use core::fmt::Write;

    for (row, line) in lines.iter().enumerate() {
        if let Ok(s) = core::str::from_utf8(line) {
            let _ = writeln!(serial_port, "{}", s);
        }
        for (col, &byte) in line.iter().enumerate() {
            // SAFETY: `vga_buffer` points to the 80x25 VGA text buffer at physical `0xb8000` (`row < 13`, `col < 80`).
            unsafe {
                *vga_buffer.offset((row * 80 + col) as isize * 2) = byte;
                *vga_buffer.offset((row * 80 + col) as isize * 2 + 1) = 0x0a;
            }
        }
    }

    loop {
        crate::net::poll_network();
        // SAFETY: Ring-0 kernel loop halting CPU until next hardware interrupt.
        unsafe { asm!("hlt") }
    }
}

/// Hard panic handler halting bare-metal execution locally on fatal error.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
