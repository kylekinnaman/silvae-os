// Sovereign module fragment.

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests APIC feature flag detection and MSR base address parsing.
    #[test]
    fn test_apic_detection_logic() {
        let edx_has_apic = 1 << 9;
        let ecx_has_x2apic = 1 << 21;
        let msr_enabled = IA32_APIC_BASE_ENABLE_BIT | 0xFEE0_0000;

        let info = ApicInfo::detect_from_features(edx_has_apic, ecx_has_x2apic, msr_enabled);
        assert!(info.has_local_apic);
        assert!(info.has_x2apic);
        assert!(info.is_globally_enabled);
        assert_eq!(info.base_address, 0xFEE0_0000);
    }

    /// Tests APIC fallback to standard physical base address when disabled.
    #[test]
    fn test_apic_fallback_base_address() {
        let edx_has_apic = 1 << 9;
        let info = ApicInfo::detect_from_features(edx_has_apic, 0, 0);
        assert!(info.has_local_apic);
        assert!(!info.has_x2apic);
        assert!(!info.is_globally_enabled);
        assert_eq!(info.base_address, DEFAULT_APIC_BASE);
    }

    /// Tests encoding and decoding roundtrip for I/O APIC redirection entries.
    #[test]
    fn test_ioapic_redirection_encoding_decoding() {
        let entry = IoApicRedirectionEntry {
            vector: 43,
            delivery_mode: 0,
            destination_mode: false,
            polarity_low: true,
            trigger_level: true,
            masked: false,
            destination_id: 3,
        };

        let (low, high) = entry.encode();
        let decoded = IoApicRedirectionEntry::decode(low, high);

        assert_eq!(decoded.vector, 43);
        assert_eq!(decoded.delivery_mode, 0);
        assert!(!decoded.destination_mode);
        assert!(decoded.polarity_low);
        assert!(decoded.trigger_level);
        assert!(!decoded.masked);
        assert_eq!(decoded.destination_id, 3);
    }

    /// Tests default bitfield states for masked I/O APIC redirection table entries.
    #[test]
    fn test_ioapic_masked_entry() {
        let entry = IoApicRedirectionEntry::new_masked(0x20);
        let (low, _) = entry.encode();
        assert!((low & (1 << 16)) != 0);
        assert_eq!((low & 0xFF) as u8, 0x20);
    }

    /// Tests global APIC status flag updates and query states.
    #[test]
    fn test_apic_flag_state() {
        set_apic_enabled(false);
        assert!(!is_apic_enabled());
        set_apic_enabled(true);
        assert!(is_apic_enabled());
        set_apic_enabled(false);
    }

    /// Tests memory-mapped Local APIC register read, write, enable, and EOI operations.
    #[test]
    fn test_local_apic_operations() {
        let mut apic_mem = [0u32; 1024];
        let base = apic_mem.as_mut_ptr() as usize;
        let apic = LocalApic::new(base);

        unsafe { // SAFETY: Verified invariant.
            apic.enable();
            assert!(is_apic_enabled());
            assert_eq!(apic.read_reg(APIC_REG_TPR), 0);
            assert_eq!(apic.read_reg(APIC_REG_SVR), APIC_SVR_SOFTWARE_ENABLE | APIC_SPURIOUS_VECTOR);

            // Populate target hardware version and ID fields in memory
            apic_mem[APIC_REG_VERSION / 4] = 0x14;
            assert_eq!(apic.version(), 0x14);

            apic_mem[APIC_REG_ID / 4] = 0x05 << 24;
            assert_eq!(apic.id(), 0x05);

            apic.send_eoi();
            assert_eq!(apic_mem[APIC_REG_EOI / 4], 0);

            local_apic_eoi(base);
            assert_eq!(apic_mem[APIC_REG_EOI / 4], 0);
        }
        set_apic_enabled(false);
    }

    /// Tests standard unmasked edge-triggered redirection table entry creation.
    #[test]
    fn test_ioapic_standard_entry() {
        let entry = IoApicRedirectionEntry::new_standard(43, 2);
        assert_eq!(entry.vector, 43);
        assert_eq!(entry.destination_id, 2);
        assert!(!entry.masked);
        assert_eq!(entry.delivery_mode, 0);
        assert!(!entry.destination_mode);
        assert!(!entry.polarity_low);
        assert!(!entry.trigger_level);
    }

    /// Tests legacy PIC masking operation.
    #[test]
    fn test_disable_legacy_pic() {
        unsafe { // SAFETY: Verified invariant.
            disable_legacy_pic();
        }
    }
}
