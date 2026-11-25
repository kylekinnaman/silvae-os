use crate::pairing::{build_welcome_banner, derive_pairing_pin, format_pairing_pin};

/// Returns calibrated timestamp counter reference value for arithmetic bound validation.
fn calibrated_tsc_reference() -> u64 {
    0x1234_5678_9ABC_DEF0
}

/// Evaluates the boundaries of the PIN generation algorithm.
#[test]
fn test_pin_generation_bounds() {
    let pin = derive_pairing_pin(calibrated_tsc_reference());
    assert!(pin < 100_000_000);
}

/// Verifies the structural string generation logic for the output sequence.
#[test]
fn test_pin_string_generation() {
    let pin = derive_pairing_pin(calibrated_tsc_reference());
    let pin_str = format_pairing_pin(pin);

    assert_eq!(pin_str[4], b' ');
    assert_eq!(pin_str.iter().filter(|&&c| c == b' ').count(), 1);

    for (i, &ch) in pin_str.iter().enumerate() {
        if i != 4 {
            assert!(ch.is_ascii_digit());
        }
    }
}

/// Checks for edge case failures related to extreme TSC values.
#[test]
fn test_pin_string_edge_cases() {
    assert_eq!(derive_pairing_pin(0), 0);
    assert!(derive_pairing_pin(u64::MAX) < 100_000_000);
}

/// Evaluates the PIN string generation limits against massive timestamp overflows.
#[test]
fn test_pin_string_overflow_behavior() {
    let pin_large = derive_pairing_pin(u64::MAX);
    assert!(pin_large < 100_000_000);
}

/// Tests the structural byte padding logic to ensure no buffer boundary overflows.
#[test]
fn test_buffer_bounds() {
    let pin_str = format_pairing_pin(derive_pairing_pin(calibrated_tsc_reference()));
    assert_eq!(pin_str.len(), 9);
    assert_eq!(pin_str[4], b' ');
}

/// Evaluates the terminal ASCII output buffer against hardcoded line segment lengths.
#[test]
fn test_line_injection_bounds() {
    let pin_str = format_pairing_pin(derive_pairing_pin(calibrated_tsc_reference()));
    let lines = build_welcome_banner(&pin_str);
    assert_eq!(&lines[11][35..44], &pin_str);
}

/// Validates the LCG behavior when the pseudo-random seed hits zero.
#[test]
fn test_zero_pin_handling() {
    let pin_zero = derive_pairing_pin(0);
    assert_eq!(pin_zero, 0);

    let pin_str = format_pairing_pin(pin_zero);
    assert_eq!(pin_str[0], b'0');
    assert_eq!(pin_str[4], b' ');
    assert_eq!(pin_str[8], b'0');
}

/// Evaluates the PIN constraint logic when reaching the absolute 64-bit maximums.
#[test]
fn test_maximum_pin_handling() {
    let pin_max = derive_pairing_pin(u64::MAX);
    assert!(pin_max < 100_000_000);

    let pin_str = format_pairing_pin(pin_max);
    assert_eq!(pin_str[4], b' ');
    for (i, &ch) in pin_str.iter().enumerate() {
        if i != 4 {
            assert!(ch.is_ascii_digit());
        }
    }
}

/// Tests extreme TSC values against the LCG modulo math to prevent exceptions.
#[test]
fn test_invalid_tsc_values() {
    let test_cases = [0, 1, u64::MAX, 0x1234_5678_9ABC_DEF0, 0xFEDC_BA98_7654_3210];
    for tsc in test_cases {
        assert!(derive_pairing_pin(tsc) < 100_000_000);
    }
}

/// Confirms that formatting outputs maintain stable offsets regardless of time variables.
#[test]
fn test_pin_string_formatting_consistency() {
    let pin_str = format_pairing_pin(derive_pairing_pin(calibrated_tsc_reference()));
    assert_eq!(pin_str[4], b' ');
    assert_eq!(pin_str.iter().filter(|&&c| c.is_ascii_digit()).count(), 8);
}

/// Checks the maximum u64 value behavior within the wrapped multiplication steps.
#[test]
fn test_no_panic_with_large_tsc() {
    assert!(derive_pairing_pin(u64::MAX) < 100_000_000);
}

/// Verifies parsing logic correctly handles formatted space-delimited digits.
#[test]
fn test_pin_string_parsing_correctness() {
    let pin_str = format_pairing_pin(derive_pairing_pin(calibrated_tsc_reference()));
    let parsed_pin: String = pin_str
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != 4)
        .map(|(_, &b)| b as char)
        .collect();

    assert_eq!(parsed_pin.len(), 8);
    assert!(parsed_pin.parse::<u32>().is_ok());
}

/// Checks that the screen presentation buffer maintains precise column dimensions.
#[test]
fn test_line_bounds_checking() {
    let lines = build_welcome_banner(b"1234 5678");
    assert_eq!(lines.len(), 13);
    for line in &lines {
        assert_eq!(line.len(), 80);
    }
}

/// Evaluates exact memory slices to prevent dangerous out-of-bounds pointer reads.
#[test]
fn test_memory_alignment() {
    let pin_str = format_pairing_pin(derive_pairing_pin(calibrated_tsc_reference()));
    let lines = build_welcome_banner(&pin_str);
    assert_eq!(&lines[11][35..44], &pin_str);
}

/// Verifies Activity 1: Interrupts and APIC configuration mappings.
#[test]
fn test_activity1_interrupts_and_apic() {
    use crate::apic::{ApicInfo, IoApicRedirectionEntry, DEFAULT_APIC_BASE};
    use crate::interrupts::{InterruptIndex, PIC_1_OFFSET, PIC_2_OFFSET};

    assert_eq!(InterruptIndex::Timer.as_u8(), PIC_1_OFFSET);
    assert_eq!(InterruptIndex::Rtl8139.as_u8(), PIC_1_OFFSET + 11);
    assert_eq!(PIC_2_OFFSET, PIC_1_OFFSET + 8);

    let edx_features = 1 << 9;
    let ecx_features = 1 << 21;
    let msr_apic = (1 << 11) | DEFAULT_APIC_BASE;
    let apic_info = ApicInfo::detect_from_features(edx_features, ecx_features, msr_apic);
    assert!(apic_info.has_local_apic);
    assert!(apic_info.has_x2apic);
    assert!(apic_info.is_globally_enabled);
    assert_eq!(apic_info.base_address, DEFAULT_APIC_BASE);

    let net_irq_entry = IoApicRedirectionEntry::new_standard(InterruptIndex::Rtl8139.as_u8(), 0);
    assert_eq!(net_irq_entry.vector, PIC_1_OFFSET + 11);
    assert!(!net_irq_entry.masked);

    crate::net::notify_network_interrupt();
    assert!(crate::net::consume_network_interrupt());
    assert!(!crate::net::consume_network_interrupt());
}

/// Verifies Activity 2: Dynamic Memory Management, Heap Boundaries, and DMA Ring Buffers.
#[test]
fn test_activity2_dma_and_allocator() {
    use crate::allocator::{heap_end, heap_size, heap_start, HEAP_SIZE};
    use crate::dma::{DmaBuffer, DmaRingBuffer, DMA_ALIGNMENT};

    assert_eq!(heap_size(), HEAP_SIZE);
    assert_eq!(heap_end(), heap_start() + heap_size());
    assert!(heap_size() >= 256 * 1024);

    let dma_buf = DmaBuffer::<512>::new();
    let addr = dma_buf.physical_address() as usize;
    assert_eq!(addr % DMA_ALIGNMENT, 0);

    let mut dma_ring = DmaRingBuffer::<2048>::new();
    let payload = b"SILVAE-ETHERNET-FRAME-PACKET";
    let next_offset = dma_ring
        .write_hardware_packet(0, 0x0001, payload)
        .expect("write failed");
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
        find_network_adapters, NetworkDriver, PciDevice, PCI_CLASS_NETWORK, PCI_CMD_BUS_MASTER,
        PCI_CMD_IO_SPACE, PCI_CMD_MEMORY_SPACE, PCI_SUBCLASS_ETHERNET,
    };

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

    let devices = alloc::vec![rtl_dev, intel_dev, virtio_dev];
    let matched = find_network_adapters(&devices);
    assert_eq!(matched.len(), 3);
    assert_eq!(matched[0].1, NetworkDriver::Realtek8139);
    assert_eq!(matched[1].1, NetworkDriver::IntelE1000);
    assert_eq!(matched[2].1, NetworkDriver::VirtIoNet);

    let required_cmd_flags = PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER;
    assert_eq!(required_cmd_flags, 0x0007);
}
