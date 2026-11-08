// Sovereign module fragment.

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests network interface controller initialization and MAC address parsing.
    #[test]
    fn test_rtl8139_lifecycle() {
        crate::port::reset_hardware_bus();
        let nic = Rtl8139::new(0xC000);
        assert_eq!(nic.mac_address, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    }

    /// Tests packet reception rejection when hardware status buffer flag is empty.
    #[test]
    fn test_rtl8139_empty_receive() {
        crate::port::reset_hardware_bus();
        let mut nic = Rtl8139::new(0xC000);
        unsafe { // SAFETY: Verified invariant.
            let mut cr = IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x01); // Buffer empty bit
        }
        assert!(nic.receive().is_none());
    }

    /// Tests packet reception with invalid length headers.
    #[test]
    fn test_rtl8139_receive_invalid_length() {
        crate::port::reset_hardware_bus();
        let mut nic = Rtl8139::new(0xC000);
        unsafe { // SAFETY: Verified invariant.
            let mut cr = IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x00);
        }
        // Buffer at index 0 has length 0
        assert!(nic.receive().is_none());

        // Length exceeding 1500
        let len_bytes = 2000u16.to_le_bytes();
        nic.rx_buffer[nic.rx_index + 2] = len_bytes[0];
        nic.rx_buffer[nic.rx_index + 3] = len_bytes[1];
        assert!(nic.receive().is_none());
    }

    /// Tests successful packet reception and transmission workflows.
    #[test]
    fn test_rtl8139_packet_reception_and_transmission() {
        crate::port::reset_hardware_bus();
        let mut nic = Rtl8139::new(0xC000);
        unsafe { // SAFETY: Verified invariant.
            let mut cr = IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x00);
        }

        // Write packet header: status = 0x0001, total_length = 14 (4 byte header + 10 byte payload)
        let status_bytes = 0x0001u16.to_le_bytes();
        let len_bytes = 14u16.to_le_bytes();
        let payload = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let idx = nic.rx_index;
        nic.rx_buffer[idx] = status_bytes[0];
        nic.rx_buffer[idx + 1] = status_bytes[1];
        nic.rx_buffer[idx + 2] = len_bytes[0];
        nic.rx_buffer[idx + 3] = len_bytes[1];
        nic.rx_buffer[idx + 4..idx + 14].copy_from_slice(&payload);

        let received = nic.receive().expect("packet must be received");
        assert_eq!(received.as_slice(), &payload);

        // Test transmission
        nic.transmit(&[0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(&nic.tx_buffer[0..4], &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    /// Tests ring wrap calculation when index exceeds buffer size limit.
    #[test]
    fn test_rtl8139_receive_ring_wrap() {
        crate::port::reset_hardware_bus();
        let mut nic = Rtl8139::new(0xC000);
        unsafe { // SAFETY: Verified invariant.
            let mut cr = IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x00);
        }

        // Assigns rx_index near ring boundary
        nic.rx_index = 8190;
        let idx = nic.rx_index;
        let len_bytes = 14u16.to_le_bytes();
        nic.rx_buffer[idx + 2] = len_bytes[0];
        nic.rx_buffer[idx + 3] = len_bytes[1];

        let _ = nic.receive();
        assert!(nic.rx_index < 8192);
    }
}
