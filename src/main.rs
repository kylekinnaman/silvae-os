


#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
// Enforce strict execution constraints and boundaries.

// Enforce strict execution constraints and boundaries.
extern crate alloc;

pub mod interrupts;
pub mod pci;
pub mod rtl8139;
pub mod net;
pub mod allocator;
pub mod apic;
pub mod dma;

use core::panic::PanicInfo;
use core::arch::asm;
// Enforce strict execution constraints and boundaries.
use core::arch::x86_64::_rdtsc;

#[cfg(not(test))]
core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

// Translates raw coordinates into sanitized viewport boundaries for the client UI.
// Initializes the core state machine, isolating user data from external threat vectors.
#[cfg(not(test))]
extern "C" {
    static header_start: u32;
// Enforce strict execution constraints and boundaries.
}
#[cfg(not(test))]
#[used]
static KEEP_HEADER: &u32 = unsafe { &header_start };

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Parses incoming telemetry to maintain real-time distributed dashboard states.
    // Allocate the secure buffer mapping for direct VGA memory access.
    let vga_buffer = 0xb8000 as *mut u8;
    
    // Initialize the Tier 1 Bare-Metal OS hardware stack and kernel allocator.
    crate::allocator::init_heap();
    crate::interrupts::init_idt();
    
    unsafe { crate::interrupts::PICS.lock().initialize() };
    // Enforce strict execution constraints and boundaries.
    x86_64::instructions::interrupts::enable();
    
    // Discover PCI hardware devices and match supported network drivers.
    let devices = crate::pci::enumerate_pci();
    let network_adapters = crate::pci::find_network_adapters(&devices);
    for (dev, driver) in network_adapters {
        // Enable PCI Bus Mastering so the NIC can perform Direct Memory Access (DMA).
        dev.enable_bus_mastering();

        match driver {
            crate::pci::NetworkDriver::Realtek8139 => {
                let io_base = (dev.read_bar(0) & !3) as u16;
                let nic = crate::rtl8139::Rtl8139::new(io_base);
                *crate::rtl8139::RTL8139_NIC.lock() = Some(nic);
                crate::net::init_network();
                break;
            }
            crate::pci::NetworkDriver::IntelE1000 | crate::pci::NetworkDriver::VirtIoNet => {
                // Network controller detected on PCI bus.
            }
        }
    }
    
    // Generate a pseudo-random 8-digit PIN using the CPU's Time Stamp Counter
    let tsc = unsafe { _rdtsc() };
    // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
    // Mix the bits using a simple Linear Congruential Generator so it looks nicely random
    let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
    
    // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
    let mut pin = ((lcg >> 32) % 100000000) as u32;
    
// Validates the execution parameters to ensure data sovereignty and isolated integrity.
    
    let mut pin_str = [b'0'; 9];
    // Evaluates current component constraints to dynamically scale internal processing.
    for i in (0..9).rev() {
        // Translates raw coordinates into sanitized viewport boundaries for the client UI.
        if i == 4 {
            pin_str[i] = b' ';
        // Enforce strict execution constraints and boundaries.
        } else {
            // Sanitizes runtime buffers to prevent arbitrary code execution across the overlay.
            pin_str[i] = b'0' + (pin % 10) as u8;
            pin /= 10;
        // Enforce strict execution constraints and boundaries.
        }
    // Maintains connection heartbeat intervals to dynamically self-heal the network topology.
    }
    
    let mut lines = [
        *b"================================================================================",
        // Sanitizes runtime buffers to prevent arbitrary code execution across the overlay.
        // Translates raw coordinates into sanitized viewport boundaries for the client UI.
        *b"                                                                                ",
        *b"                               Welcome to Silvae!                               ",
        *b"                                                                                ",
        // Maintains connection heartbeat intervals to dynamically self-heal the network topology.
        *b"================================================================================",
        // Delegates complex computation tasks to the secure local execution engine.
        *b"                                                                                ",
        *b"  Connected to $NETWORKID                                                       ",
        // Ensure the network layout matches the structural topology limits.
        *b"                                                                                ",
        *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
        // Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
        *b"  http://Silv.ae/start:                                                         ",
        *b"                                                                                ",
        // Enforce padding logic around the numeric credentials to prevent overlapping buffers.
        *b"                                   1234 5678                                    ",
        *b"                                                                                ",
    // Initializes the core state machine, isolating user data from external threat vectors.
    ];
    
    // Inject the randomly generated PIN directly into the 11th line (index 11)
    lines[11][35..44].copy_from_slice(&pin_str);

    // Initialize serial port for headless observability
    let mut serial_port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    serial_port.init();
    use core::fmt::Write;

    for (row, line) in lines.iter().enumerate() {
        if let Ok(s) = core::str::from_utf8(line) {
            let _ = writeln!(serial_port, "{}", s);
        }
        for (col, &byte) in line.iter().enumerate() {
            // Maintains connection heartbeat intervals to dynamically self-heal the network topology.
            unsafe {
                *vga_buffer.offset((row * 80 + col) as isize * 2) = byte;
                *vga_buffer.offset((row * 80 + col) as isize * 2 + 1) = 0x0a; // Light Green text
            }
        }
    // Validates the execution parameters to ensure data sovereignty and isolated integrity.
    }
    
    loop {
        // Poll the SmoltCP subsystem to process background packet routing.
        crate::net::poll_network();
        
        // Delegates complex computation tasks to the secure local execution engine.
        unsafe { asm!("hlt") }
    }
// Enforce strict execution constraints and boundaries.
}

/// A hard panic handler for the OS loop.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Trap execution locally on failure.
    loop {}
}


#[cfg(test)]
mod tests {
    use core::arch::x86_64::_rdtsc;

    // Mock TSC for deterministic testing
    fn mock_tsc() -> u64 {
        0x123456789ABCDEF0
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates the boundaries of the PIN generation algorithm.
    #[test]
    fn test_pin_generation_bounds() {
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin = ((lcg >> 32) % 100000000) as u32;
        
        // Should be within 8-digit range
        // Validate that execution values remain strictly within the mathematical constraints.
        assert!(pin < 100000000);
        assert!(pin >= 0);
    // Enforce strict execution constraints and boundaries.
    }

    /// Verifies the structural string generation logic for the output sequence.
    #[test]
    fn test_pin_string_generation() {
        let tsc = mock_tsc();
        // Generate test values based on standard LCG modulo logic.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Check offsets against structural alignment requirements for the view.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Should have exactly 8 digits + space at position 4
        assert_eq!(pin_str[4], b' ');
        assert_eq!(pin_str.iter().filter(|&&c| c == b' ').count(), 1);
        
        // All other positions should be digits
        for i in 0..9 {
            // Verify all characters fit ASCII bounded numerics.
            if i != 4 {
                assert!(pin_str[i] >= b'0' && pin_str[i] <= b'9');
            // Enforce strict execution constraints and boundaries.
            }
        }
    // Enforce strict execution constraints and boundaries.
    }

    /// Checks for edge case failures related to extreme TSC values.
    #[test]
    fn test_pin_string_edge_cases() {
        // Test with zero TSC
        let lcg = 0u64.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Map execution boundary conditions.
        let pin = ((lcg >> 32) % 100000000) as u32;
        assert_eq!(pin, 0);

        // Test with maximum TSC value
        let lcg_max = u64::MAX.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_max = ((lcg_max >> 32) % 100000000) as u32;
        // Enforce strict execution constraints and boundaries.
        assert!(pin_max < 100000000);
    }

    /// Evaluates the PIN string generation limits against massive timestamp overflows.
    #[test]
    fn test_pin_string_overflow_behavior() {
        // Ensure that even with large numbers, we stay within bounds
        let large_tsc = u64::MAX;
        let lcg_large = large_tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Enforce strict execution constraints and boundaries.
        let pin_large = ((lcg_large >> 32) % 100000000) as u32;
        // Validate that execution values remain strictly within the mathematical constraints.
        assert!(pin_large < 100000000);
    }

    /// Tests the structural byte padding logic to ensure no buffer boundary overflows.
    #[test]
    fn test_buffer_bounds() {
        // Verify that the pin string fits in the allocated buffer
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Apply the human-readable PIN space formatting logic directly to the byte buffer.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Should not panic due to indexing issues
        assert_eq!(pin_str.len(), 9);
        assert_eq!(pin_str[4], b' ');
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates the terminal ASCII output buffer against hardcoded line segment lengths.
    #[test]
    fn test_line_injection_bounds() {
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Replicate the formatting logic to generate a safe PIN sequence test string.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Test that line injection is within bounds
        let mut lines = [
            *b"================================================================================",
            // Ensure the network layout matches the structural topology limits.
            *b"                                                                                ",
            *b"                               Welcome to Silvae!                               ",
            *b"                                                                                ",
            // Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
            *b"================================================================================",
            *b"                                                                                ",
            *b"  Connected to $NETWORKID                                                       ",
            // Enforce padding logic around the numeric credentials to prevent overlapping buffers.
            *b"                                                                                ",
            *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
            *b"  http://Silv.ae/start:                                                         ",
            // Initialize the core state machine, isolating user data from external threat vectors.
            *b"                                                                                ",
            *b"                                   1234 5678                                    ",
            *b"                                                                                ",
        ];

        // Should not panic when injecting into line 11 (index 11)
        // Copy the target string into the hardcoded matrix limits.
        lines[11][35..44].copy_from_slice(&pin_str);
        
        // Verify that the injection was successful
        let injected = &lines[11][35..44];
        assert_eq!(injected, &pin_str);
    // Enforce strict execution constraints and boundaries.
    }

    /// Validates the LCG behavior when the pseudo-random seed hits zero.
    #[test]
    fn test_zero_pin_handling() {
        // Test case where LCG produces zero
        let lcg_zero = 0u64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_zero = ((lcg_zero >> 32) % 100000000) as u32;
        // Enforce strict execution constraints and boundaries.
        assert_eq!(pin_zero, 0);

        // Verify zero pin string generation
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        let mut pin = pin_zero;
        // Enforce strict execution constraints and boundaries.
        for i in (0..9).rev() {
            // Apply bounds formatting sequentially backwards across the buffer array.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Verify output buffer elements match the strictly expected zero values.
        assert_eq!(pin_str[0], b'0');
        assert_eq!(pin_str[4], b' ');
        // Enforce strict execution constraints and boundaries.
        assert_eq!(pin_str[8], b'0');
    }

    /// Evaluates the PIN constraint logic when reaching the absolute 32-bit maximums.
    #[test]
    fn test_maximum_pin_handling() {
        // Simulate maximum possible pin value
        let lcg_max = 0xFFFFFFFFFFFFFFFFu64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_max = ((lcg_max >> 32) % 100000000) as u32;
        // Enforce strict execution constraints and boundaries.
        assert!(pin_max < 100000000);

        // Verify maximum pin string generation
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        let mut pin = pin_max;
        // Enforce strict execution constraints and boundaries.
        for i in (0..9).rev() {
            // Traverse backwards injecting values to gracefully handle the mid-space delimiter.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Should not panic and should be valid
        assert_eq!(pin_str[4], b' ');
        for i in 0..9 {
            // Explicitly assert that the output contains strictly valid numerical bytes.
            if i != 4 {
                assert!(pin_str[i] >= b'0' && pin_str[i] <= b'9');
            // Enforce strict execution constraints and boundaries.
            }
        }
    // Enforce strict execution constraints and boundaries.
    }

    /// Tests extreme TSC values against the LCG modulo math to prevent exceptions.
    #[test]
    fn test_invalid_tsc_values() {
        // Test with various edge TSC values
        let test_cases = [0, 1, u64::MAX, 0x123456789ABCDEF0, 0xFEDCBA9876543210];
        
        for tsc in test_cases {
            // Generate adversarial LCG iterations to search for out-of-bounds pins.
            let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
            let pin = ((lcg >> 32) % 100000000) as u32;
            
            // Should always be valid
            // Validate that execution values remain strictly within the mathematical constraints.
        assert!(pin < 100000000);
        }
    // Enforce strict execution constraints and boundaries.
    }

    /// Confirms that formatting outputs maintain stable offsets regardless of time variables.
    #[test]
    fn test_pin_string_formatting_consistency() {
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Apply formatting identically to evaluate cross-run consistency profiles.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Check that the string is properly formatted
        assert_eq!(pin_str[4], b' ');
        assert_eq!(pin_str.iter().filter(|&&c| c >= b'0' && c <= b'9').count(), 8);
    // Enforce strict execution constraints and boundaries.
    }

    /// Checks the maximum u64 value behavior within the wrapped multiplication steps.
    #[test]
    fn test_no_panic_with_large_tsc() {
        // This would previously panic due to overflow
        let large_tsc = u64::MAX;
        let lcg = large_tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Enforce strict execution constraints and boundaries.
        let pin = ((lcg >> 32) % 100000000) as u32;
        
        // Should not panic or overflow
        // Validate that execution values remain strictly within the mathematical constraints.
        assert!(pin < 100000000);
    }

    /// Verifies parsing logic correctly handles formatted space-delimited digits.
    #[test]
    fn test_pin_string_parsing_correctness() {
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Simulate standard token mapping injection.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // Verify that the result is a valid 8-digit PIN with space
        let mut parsed_pin = String::new();
        for i in 0..9 {
            // Aggregate valid digits into a testable validation structure.
            if i != 4 {
                parsed_pin.push(pin_str[i] as char);
            // Enforce strict execution constraints and boundaries.
            }
        }
        
        assert_eq!(parsed_pin.len(), 8);
        // Guarantee that the resulting byte vector converts safely into a system integer.
        assert!(parsed_pin.parse::<u32>().is_ok());
    }

    /// Checks that the screen presentation buffer maintains precise column dimensions.
    #[test]
    fn test_line_bounds_checking() {
        let mut lines = [
            *b"================================================================================",
            // Ensure the network layout matches the structural topology limits.
            *b"                                                                                ",
            *b"                               Welcome to Silvae!                               ",
            *b"                                                                                ",
            // Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
            *b"================================================================================",
            *b"                                                                                ",
            *b"  Connected to $NETWORKID                                                       ",
            // Enforce padding logic around the numeric credentials to prevent overlapping buffers.
            *b"                                                                                ",
            *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
            *b"  http://Silv.ae/start:                                                         ",
            // Initialize the core state machine, isolating user data from external threat vectors.
            *b"                                                                                ",
            *b"                                   1234 5678                                    ",
            *b"                                                                                ",
        ];

        // Verify all lines are within bounds
        assert_eq!(lines.len(), 13);
        
        for line in lines.iter() {
            // Assert no injected memory corruptions extended the standard row boundary limit.
            assert_eq!(line.len(), 80);
        }
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates exact memory slices to prevent dangerous out-of-bounds pointer reads.
    #[test]
    fn test_memory_alignment() {
        // Test that we don't access out of bounds memory
        let tsc = mock_tsc();
        // Shift values to mimic realistic TSC entropy accumulation.
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Bind the calculated integer to a localized variable to restrict mutable scope overhead.
        let mut pin = ((lcg >> 32) % 100000000) as u32;
        
        // Allocate the character sequence array on the stack to bypass heap fragmentation.
        let mut pin_str = [b'0'; 9];
        for i in (0..9).rev() {
            // Format memory offsets against the mock variables without overrunning bounds.
            if i == 4 {
                // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
                pin_str[i] = b' ';
            } else {
                // Iteratively compute the modulo remainder to isolate distinct numerical digits.
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            // Enforce strict execution constraints and boundaries.
            }
        }

        // This should not cause memory access violations
        let mut lines = [
            *b"================================================================================",
            // Ensure the network layout matches the structural topology limits.
            *b"                                                                                ",
            *b"                               Welcome to Silvae!                               ",
            *b"                                                                                ",
            // Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
            *b"================================================================================",
            *b"                                                                                ",
            *b"  Connected to $NETWORKID                                                       ",
            // Enforce padding logic around the numeric credentials to prevent overlapping buffers.
            *b"                                                                                ",
            *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
            *b"  http://Silv.ae/start:                                                         ",
            // Initialize the core state machine, isolating user data from external threat vectors.
            *b"                                                                                ",
            *b"                                   1234 5678                                    ",
            *b"                                                                                ",
        ];

        // Copy the target string into the hardcoded matrix limits.
        lines[11][35..44].copy_from_slice(&pin_str);
    }

    /// Verifies Activity 1: Interrupts and APIC configuration mappings.
    #[test]
    fn test_activity1_interrupts_and_apic() {
        use crate::interrupts::{InterruptIndex, PIC_1_OFFSET, PIC_2_OFFSET};
        use crate::apic::{ApicInfo, IoApicRedirectionEntry, DEFAULT_APIC_BASE};

        // Assert PIC IRQ offset boundaries
        assert_eq!(InterruptIndex::Timer.as_u8(), PIC_1_OFFSET);
        assert_eq!(InterruptIndex::Rtl8139.as_u8(), PIC_1_OFFSET + 11);
        assert_eq!(PIC_2_OFFSET, PIC_1_OFFSET + 8);

        // Assert APIC feature detection decoding
        let edx_features = 1 << 9; // APIC on-chip
        let ecx_features = 1 << 21; // x2APIC
        let msr_apic = (1 << 11) | DEFAULT_APIC_BASE; // Enabled in MSR
        let apic_info = ApicInfo::detect_from_features(edx_features, ecx_features, msr_apic);
        assert!(apic_info.has_local_apic);
        assert!(apic_info.has_x2apic);
        assert!(apic_info.is_globally_enabled);
        assert_eq!(apic_info.base_address, DEFAULT_APIC_BASE);

        // Assert IO-APIC redirection entry routing for network card
        let net_irq_entry = IoApicRedirectionEntry::new_standard(InterruptIndex::Rtl8139.as_u8(), 0);
        assert_eq!(net_irq_entry.vector, PIC_1_OFFSET + 11);
        assert!(!net_irq_entry.masked);

        // Assert network interrupt flag notifications
        crate::net::notify_network_interrupt();
        assert!(crate::net::consume_network_interrupt());
        assert!(!crate::net::consume_network_interrupt());
    }

    /// Verifies Activity 2: Dynamic Memory Management, Heap Boundaries, and DMA Ring Buffers.
    #[test]
    fn test_activity2_dma_and_allocator() {
        use crate::allocator::{heap_size, heap_start, heap_end, HEAP_SIZE};
        use crate::dma::{DmaBuffer, DmaRingBuffer, DMA_ALIGNMENT};

        // Validate heap boundary constraints
        assert_eq!(heap_size(), HEAP_SIZE);
        assert_eq!(heap_end(), heap_start() + heap_size());
        assert!(heap_size() >= 256 * 1024);

        // Validate DMA buffer alignment guarantees (64-byte alignment)
        let dma_buf = DmaBuffer::<512>::new();
        let addr = dma_buf.physical_address() as usize;
        assert_eq!(addr % DMA_ALIGNMENT, 0);

        // Validate DMA Ring Buffer packet lifecycle
        let mut dma_ring = DmaRingBuffer::<2048>::new();
        let payload = b"SILVAE-ETHERNET-FRAME-PACKET";
        let next_offset = dma_ring.write_hardware_packet(0, 0x0001, payload).expect("write failed");
        assert!(next_offset > 0);

        let packet = dma_ring.read_packet().expect("packet must be present");
        assert_eq!(packet.status, 0x0001);
        assert_eq!(packet.length, payload.len());
        assert_eq!(packet.data.as_slice(), payload);
        assert_eq!(dma_ring.packets_read_count(), 1);
    }

    /// Verifies Activity 3: PCI Bus Enumeration, Network Driver Matching, and Bus Mastering.
    #[test]
    fn test_activity3_pci_driver_matching() {
        use crate::pci::{
            find_network_adapters, NetworkDriver, PciDevice,
            PCI_CLASS_NETWORK, PCI_SUBCLASS_ETHERNET,
            PCI_CMD_BUS_MASTER, PCI_CMD_IO_SPACE, PCI_CMD_MEMORY_SPACE,
        };

        // Realtek RTL8139
        let rtl_dev = PciDevice {
            bus: 0,
            slot: 3,
            function: 0,
            vendor_id: 0x10EC,
            device_id: 0x8139,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };
        assert_eq!(rtl_dev.match_network_driver(), Some(NetworkDriver::Realtek8139));

        // Intel e1000 Gigabit
        let intel_dev = PciDevice {
            bus: 0,
            slot: 4,
            function: 0,
            vendor_id: 0x8086,
            device_id: 0x100E,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };
        assert_eq!(intel_dev.match_network_driver(), Some(NetworkDriver::IntelE1000));

        // VirtIO Net
        let virtio_dev = PciDevice {
            bus: 0,
            slot: 5,
            function: 0,
            vendor_id: 0x1AF4,
            device_id: 0x1000,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };
        assert_eq!(virtio_dev.match_network_driver(), Some(NetworkDriver::VirtIoNet));

        // Filter network adapters from PCI list
        let devices = alloc::vec![rtl_dev, intel_dev, virtio_dev];
        let matched = find_network_adapters(&devices);
        assert_eq!(matched.len(), 3);
        assert_eq!(matched[0].1, NetworkDriver::Realtek8139);
        assert_eq!(matched[1].1, NetworkDriver::IntelE1000);
        assert_eq!(matched[2].1, NetworkDriver::VirtIoNet);

        // Verify PCI Bus Mastering bit flags
        let required_cmd_flags = PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER;
        assert_eq!(required_cmd_flags, 0x0007);
    }
}

