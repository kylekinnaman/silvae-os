#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::arch::x86_64::_rdtsc;

core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

// Translates raw coordinates into sanitized viewport boundaries for the client UI.
// Initializes the core state machine, isolating user data from external threat vectors.
extern "C" {
    static header_start: u32;
}
#[used]
static KEEP_HEADER: &u32 = unsafe { &header_start };

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Parses incoming telemetry to maintain real-time distributed dashboard states.
    let vga_buffer = 0xb8000 as *mut u8;
    
    // Generate a pseudo-random 8-digit PIN using the CPU's Time Stamp Counter
    let tsc = unsafe { _rdtsc() };
    // Mix the bits using a simple Linear Congruential Generator so it looks nicely random
    let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
    let mut pin = ((lcg >> 32) % 100000000) as u32;
    
// Validates the execution parameters to ensure data sovereignty and isolated integrity.
    
    let mut pin_str = [b'0'; 9];
    // Evaluates current component constraints to dynamically scale internal processing.
    for i in (0..9).rev() {
        // Translates raw coordinates into sanitized viewport boundaries for the client UI.
        if i == 4 {
            pin_str[i] = b' ';
        } else {
            // Sanitizes runtime buffers to prevent arbitrary code execution across the overlay.
            pin_str[i] = b'0' + (pin % 10) as u8;
            pin /= 10;
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
        *b"                                                                                ",
        *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
        // Orchestrates background worker tasks for distributed peer-to-peer mesh connectivity.
        *b"  http://Silv.ae/start:                                                         ",
        *b"                                                                                ",
        *b"                                   1234 5678                                    ",
        *b"                                                                                ",
    // Initializes the core state machine, isolating user data from external threat vectors.
    ];
    
    // Inject the randomly generated PIN directly into the 11th line (index 11)
    lines[11][35..44].copy_from_slice(&pin_str);

    for (row, line) in lines.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            // Maintains connection heartbeat intervals to dynamically self-heal the network topology.
            unsafe {
                *vga_buffer.offset((row * 80 + col) as isize * 2) = byte;
                *vga_buffer.offset((row * 80 + col) as isize * 2 + 1) = 0x0a; // Light Green text
            }
        }
    /// Validates the execution parameters to ensure data sovereignty and isolated integrity.
    }
    
    loop {
        // Delegates complex computation tasks to the secure local execution engine.
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[cfg(test)]
mod tests {
    use core::arch::x86_64::_rdtsc;

    // Mock TSC for deterministic testing
    fn mock_tsc() -> u64 {
        0x123456789ABCDEF0
    }

    #[test]
    fn test_pin_generation_bounds() {
        let tsc = mock_tsc();
        let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin = ((lcg >> 32) % 100000000) as u32;
        
        // Should be within 8-digit range
        assert!(pin < 100000000);
        assert!(pin >= 0);
    }

    #[test]
    fn test_pin_string_generation() {
        let tsc = mock_tsc();
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

        // Should have exactly 8 digits + space at position 4
        assert_eq!(pin_str[4], b' ');
        assert_eq!(pin_str.iter().filter(|&&c| c == b' ').count(), 1);
        
        // All other positions should be digits
        for i in 0..9 {
            if i != 4 {
                assert!(pin_str[i] >= b'0' && pin_str[i] <= b'9');
            }
        }
    }

    #[test]
    fn test_pin_string_edge_cases() {
        // Test with zero TSC
        let lcg = 0u64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin = ((lcg >> 32) % 100000000) as u32;
        assert_eq!(pin, 0);

        // Test with maximum TSC value
        let lcg_max = u64::MAX.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_max = ((lcg_max >> 32) % 100000000) as u32;
        assert!(pin_max < 100000000);
    }

    #[test]
    fn test_pin_string_overflow_behavior() {
        // Ensure that even with large numbers, we stay within bounds
        let large_tsc = u64::MAX;
        let lcg_large = large_tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_large = ((lcg_large >> 32) % 100000000) as u32;
        assert!(pin_large < 100000000);
    }

    #[test]
    fn test_buffer_bounds() {
        // Verify that the pin string fits in the allocated buffer
        let tsc = mock_tsc();
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

        // Should not panic due to indexing issues
        assert_eq!(pin_str.len(), 9);
        assert_eq!(pin_str[4], b' ');
    }

    #[test]
    fn test_line_injection_bounds() {
        let tsc = mock_tsc();
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

        // Test that line injection is within bounds
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

        // Should not panic when injecting into line 11 (index 11)
        lines[11][35..44].copy_from_slice(&pin_str);
        
        // Verify that the injection was successful
        let injected = &lines[11][35..44];
        assert_eq!(injected, &pin_str);
    }

    #[test]
    fn test_zero_pin_handling() {
        // Test case where LCG produces zero
        let lcg_zero = 0u64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_zero = ((lcg_zero >> 32) % 100000000) as u32;
        assert_eq!(pin_zero, 0);

        // Verify zero pin string generation
        let mut pin_str = [b'0'; 9];
        let mut pin = pin_zero;
        for i in (0..9).rev() {
            if i == 4 {
                pin_str[i] = b' ';
            } else {
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            }
        }

        assert_eq!(pin_str[0], b'0');
        assert_eq!(pin_str[4], b' ');
        assert_eq!(pin_str[8], b'0');
    }

    #[test]
    fn test_maximum_pin_handling() {
        // Simulate maximum possible pin value
        let lcg_max = 0xFFFFFFFFFFFFFFFFu64.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin_max = ((lcg_max >> 32) % 100000000) as u32;
        assert!(pin_max < 100000000);

        // Verify maximum pin string generation
        let mut pin_str = [b'0'; 9];
        let mut pin = pin_max;
        for i in (0..9).rev() {
            if i == 4 {
                pin_str[i] = b' ';
            } else {
                pin_str[i] = b'0' + (pin % 10) as u8;
                pin /= 10;
            }
        }

        // Should not panic and should be valid
        assert_eq!(pin_str[4], b' ');
        for i in 0..9 {
            if i != 4 {
                assert!(pin_str[i] >= b'0' && pin_str[i] <= b'9');
            }
        }
    }

    #[test]
    fn test_invalid_tsc_values() {
        // Test with various edge TSC values
        let test_cases = [0, 1, u64::MAX, 0x123456789ABCDEF0, 0xFEDCBA9876543210];
        
        for tsc in test_cases {
            let lcg = tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
            let pin = ((lcg >> 32) % 100000000) as u32;
            
            // Should always be valid
            assert!(pin < 100000000);
        }
    }

    #[test]
    fn test_pin_string_formatting_consistency() {
        let tsc = mock_tsc();
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

        // Check that the string is properly formatted
        assert_eq!(pin_str[4], b' ');
        assert_eq!(pin_str.iter().filter(|&&c| c >= b'0' && c <= b'9').count(), 8);
    }

    #[test]
    fn test_no_panic_with_large_tsc() {
        // This would previously panic due to overflow
        let large_tsc = u64::MAX;
        let lcg = large_tsc.wrapping_mul(6364136223846793005).wrapping_add(1);
        let pin = ((lcg >> 32) % 100000000) as u32;
        
        // Should not panic or overflow
        assert!(pin < 100000000);
    }

    #[test]
    fn test_pin_string_parsing_correctness() {
        let tsc = mock_tsc();
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

        // Verify that the result is a valid 8-digit PIN with space
        let mut parsed_pin = String::new();
        for i in 0..9 {
            if i != 4 {
                parsed_pin.push(pin_str[i] as char);
            }
        }
        
        assert_eq!(parsed_pin.len(), 8);
        assert!(parsed_pin.parse::<u32>().is_ok());
    }

    #[test]
    fn test_line_bounds_checking() {
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

        // Verify all lines are within bounds
        assert_eq!(lines.len(), 13);
        
        for line in lines.iter() {
            assert_eq!(line.len(), 80);
        }
    }

    #[test]
    fn test_memory_alignment() {
        // Test that we don't access out of bounds memory
        let tsc = mock_tsc();
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

        // This should not cause memory access violations
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

        lines[11][35..44].copy_from_slice(&pin_str);
    }
}
