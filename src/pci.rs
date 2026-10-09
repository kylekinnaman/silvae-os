/// Core PCI bus enumeration and configuration space parsing.
use x86_64::instructions::port::Port;
use alloc::vec::Vec;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

/// PCI Class Code for Network Controllers.
pub const PCI_CLASS_NETWORK: u8 = 0x02;
/// PCI Subclass Code for Ethernet Controllers.
pub const PCI_SUBCLASS_ETHERNET: u8 = 0x00;

/// PCI Command Register offset.
pub const PCI_COMMAND_OFFSET: u8 = 0x04;
/// Command bit 0: I/O Space Enable.
pub const PCI_CMD_IO_SPACE: u16 = 1 << 0;
/// Command bit 1: Memory Space Enable.
pub const PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
/// Command bit 2: Bus Master Enable (Required for DMA).
pub const PCI_CMD_BUS_MASTER: u16 = 1 << 2;

/// Supported network interface controller drivers in the Silvae kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDriver {
    /// Realtek RTL8139 Fast Ethernet controller.
    Realtek8139,
    /// Intel PRO/1000 Gigabit Ethernet controller family (e1000 / e1000e).
    IntelE1000,
    /// VirtIO Paravirtualized Network Device.
    VirtIoNet,
}

impl NetworkDriver {
    /// Returns the human-readable description of the network driver.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Realtek8139 => "Realtek RTL8139 Fast Ethernet",
            Self::IntelE1000 => "Intel Gigabit Ethernet (e1000)",
            Self::VirtIoNet => "VirtIO Paravirtualized Network",
        }
    }
}

/// Represents a validated hardware device found on the PCI bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciDevice {
    /// PCI Bus number (0..=255).
    pub bus: u8,
    /// PCI Device/Slot number (0..31).
    pub slot: u8,
    /// PCI Function number (0..7).
    pub function: u8,
    /// Vendor identification number.
    pub vendor_id: u16,
    /// Device identification number.
    pub device_id: u16,
    /// PCI Class category.
    pub class: u8,
    /// PCI Subclass category.
    pub subclass: u8,
}

impl PciDevice {
    /// Extract the raw 32-bit configuration word from the target hardware offset.
    pub fn read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address = 1u32 << 31
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC);
            
        let mut port_addr = Port::<u32>::new(CONFIG_ADDRESS);
        let mut port_data = Port::<u32>::new(CONFIG_DATA);

        unsafe {
            port_addr.write(address);
            port_data.read()
        }
    }

    /// Writes a 32-bit configuration word to the target hardware offset.
    pub fn write_word(bus: u8, slot: u8, func: u8, offset: u8, value: u32) {
        let address = 1u32 << 31
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC);

        let mut port_addr = Port::<u32>::new(CONFIG_ADDRESS);
        let mut port_data = Port::<u32>::new(CONFIG_DATA);

        unsafe {
            port_addr.write(address);
            port_data.write(value);
        }
    }

    /// Read the BAR (Base Address Register) for memory-mapped operations.
    pub fn read_bar(&self, bar_index: u8) -> u32 {
        let offset = 0x10 + (bar_index * 4);
        Self::read_word(self.bus, self.slot, self.function, offset)
    }

    /// Enables Bus Master DMA, I/O Space, and Memory Space in the PCI Command register.
    pub fn enable_bus_mastering(&self) {
        let current_cmd = Self::read_word(self.bus, self.slot, self.function, PCI_COMMAND_OFFSET);
        let new_cmd = current_cmd | (PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER) as u32;
        Self::write_word(self.bus, self.slot, self.function, PCI_COMMAND_OFFSET, new_cmd);
    }

    /// Returns true if the device is categorized as a Network Controller.
    pub fn is_network_controller(&self) -> bool {
        self.class == PCI_CLASS_NETWORK
    }

    /// Matches the device hardware identifiers against supported network drivers.
    pub fn match_network_driver(&self) -> Option<NetworkDriver> {
        match (self.vendor_id, self.device_id) {
            // Realtek Semiconductor Co., Ltd. RTL-8100/8101L/8139 PCI Fast Ethernet
            (0x10EC, 0x8139) => Some(NetworkDriver::Realtek8139),

            // Intel Corporation 82540EM, 82545EM, 82574L, I211, etc.
            (0x8086, 0x100E) // 82540EM Gigabit Ethernet (standard QEMU)
            | (0x8086, 0x100F) // 82545EM Gigabit Ethernet
            | (0x8086, 0x10D3) // 82574L Gigabit Network Connection
            | (0x8086, 0x1539) // I211 Gigabit Network Connection
            | (0x8086, 0x107C) // 82541PI Gigabit Ethernet
            | (0x8086, 0x109A) // 82573L Gigabit Ethernet
            => Some(NetworkDriver::IntelE1000),

            // Red Hat, Inc. / VirtIO Network Device
            (0x1AF4, 0x1000) // VirtIO legacy network device
            | (0x1AF4, 0x1041) // VirtIO 1.0+ transitional network device
            => Some(NetworkDriver::VirtIoNet),

            _ => None,
        }
    }
}

/// Identifies all compatible network controllers discovered on the PCI bus.
pub fn find_network_adapters(devices: &[PciDevice]) -> Vec<(PciDevice, NetworkDriver)> {
    let mut adapters = Vec::new();
    for dev in devices {
        if let Some(driver) = dev.match_network_driver() {
            adapters.push((dev.clone(), driver));
        }
    }
    adapters
}

/// Scans the entire PCI bus layout to map present hardware components into the system.
pub fn enumerate_pci() -> Vec<PciDevice> {
    let mut devices = alloc::vec::Vec::new();

    for bus in 0..=255 {
        for slot in 0..32 {
            for function in 0..8 {
                let vendor_id = (PciDevice::read_word(bus, slot, function, 0) & 0xFFFF) as u16;
                if vendor_id == 0xFFFF {
                    continue;
                }

                let device_id = (PciDevice::read_word(bus, slot, function, 0) >> 16) as u16;
                let class_word = PciDevice::read_word(bus, slot, function, 0x08);
                let class = (class_word >> 24) as u8;
                let subclass = ((class_word >> 16) & 0xFF) as u8;

                devices.push(PciDevice {
                    bus,
                    slot,
                    function,
                    vendor_id,
                    device_id,
                    class,
                    subclass,
                });
            }
        }
    }

    devices
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_pci_command_bits() {
        let flags = PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER;
        assert_eq!(flags, 0x0007);
    }
}
