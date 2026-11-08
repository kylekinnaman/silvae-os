// Sovereign module fragment.

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
