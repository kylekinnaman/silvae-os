// Sovereign module fragment.

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests driver matching for Realtek network controller devices.
    #[test]
    fn test_driver_matching_realtek() {
        let dev = PciDevice {
            bus: 0,
            slot: 3,
            function: 0,
            vendor_id: 0x10EC,
            device_id: 0x8139,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };

        assert!(dev.is_network_controller());
        assert_eq!(dev.match_network_driver(), Some(NetworkDriver::Realtek8139));
        assert_eq!(NetworkDriver::Realtek8139.name(), "Realtek RTL8139 Fast Ethernet");
    }

    /// Tests driver matching for Intel e1000 network controller devices.
    #[test]
    fn test_driver_matching_intel_e1000() {
        let dev = PciDevice {
            bus: 0,
            slot: 3,
            function: 0,
            vendor_id: 0x8086,
            device_id: 0x100E, // QEMU default 82540EM
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };

        assert!(dev.is_network_controller());
        assert_eq!(dev.match_network_driver(), Some(NetworkDriver::IntelE1000));
        assert_eq!(NetworkDriver::IntelE1000.name(), "Intel Gigabit Ethernet (e1000)");
    }

    /// Tests driver matching for VirtIO network controller devices.
    #[test]
    fn test_driver_matching_virtio() {
        let dev = PciDevice {
            bus: 0,
            slot: 4,
            function: 0,
            vendor_id: 0x1AF4,
            device_id: 0x1000,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };

        assert!(dev.is_network_controller());
        assert_eq!(dev.match_network_driver(), Some(NetworkDriver::VirtIoNet));
        assert_eq!(NetworkDriver::VirtIoNet.name(), "VirtIO Paravirtualized Network");
    }

    /// Tests driver matching failure for non-network PCI devices.
    #[test]
    fn test_driver_matching_unknown_device() {
        let dev = PciDevice {
            bus: 0,
            slot: 2,
            function: 0,
            vendor_id: 0x1234,
            device_id: 0x5678,
            class: 0x03, // Display controller
            subclass: 0x00,
        };

        assert!(!dev.is_network_controller());
        assert_eq!(dev.match_network_driver(), None);
    }

    /// Tests filtering of device lists to extract network adapter candidates.
    #[test]
    fn test_find_network_adapters_filter() {
        let devices = alloc::vec![
            PciDevice {
                bus: 0,
                slot: 1,
                function: 0,
                vendor_id: 0x8086,
                device_id: 0x7111, // IDE controller
                class: 0x01,
                subclass: 0x01,
            },
            PciDevice {
                bus: 0,
                slot: 3,
                function: 0,
                vendor_id: 0x10EC,
                device_id: 0x8139,
                class: PCI_CLASS_NETWORK,
                subclass: PCI_SUBCLASS_ETHERNET,
            },
        ];

        let adapters = find_network_adapters(&devices);
        assert_eq!(adapters.len(), 1);
        assert_eq!(adapters[0].1, NetworkDriver::Realtek8139);
        assert_eq!(adapters[0].0.vendor_id, 0x10EC);
    }

    /// Tests PCI command bitfield flag definitions.
    #[test]
    fn test_pci_command_bits() {
        let flags = PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER;
        assert_eq!(flags, 0x0007);
    }

    /// Tests configuration word reads, writes, BAR mapping, and bus mastering.
    #[test]
    fn test_pci_read_write_word_and_bar() {
        crate::port::reset_hardware_bus();
        let dev = PciDevice {
            bus: 0,
            slot: 3,
            function: 0,
            vendor_id: 0x10EC,
            device_id: 0x8139,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };
        crate::port::register_pci_devices(alloc::vec![dev.clone()]);

        let vendor = (PciDevice::read_word(0, 3, 0, 0x00) & 0xFFFF) as u16;
        assert_eq!(vendor, 0x10EC);

        let bar0 = dev.read_bar(0);
        assert_eq!(bar0, 0xC001);

        dev.enable_bus_mastering();
        let cmd = PciDevice::read_word(0, 3, 0, PCI_COMMAND_OFFSET);
        assert_eq!(cmd & 0x07, 0x07);

        crate::port::reset_hardware_bus();
    }

    /// Tests hardware bus enumeration and device parsing.
    #[test]
    fn test_pci_enumeration() {
        crate::port::reset_hardware_bus();
        let dev = PciDevice {
            bus: 0,
            slot: 3,
            function: 0,
            vendor_id: 0x10EC,
            device_id: 0x8139,
            class: PCI_CLASS_NETWORK,
            subclass: PCI_SUBCLASS_ETHERNET,
        };
        crate::port::register_pci_devices(alloc::vec![dev.clone()]);

        let devices = enumerate_pci();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].vendor_id, 0x10EC);
        assert_eq!(devices[0].device_id, 0x8139);

        crate::port::reset_hardware_bus();
    }
}
