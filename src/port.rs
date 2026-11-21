/// Hardware I/O port communication layer.
use core::marker::PhantomData;

#[cfg(test)]
use spin::Mutex;
#[cfg(test)]
use lazy_static::lazy_static;

#[cfg(test)]
/// Global software register bus for I/O operations during testing.
pub struct HardwareRegisterBus {
    /// Target PCI configuration address register.
    pub pci_address: u32,
    /// Discovered PCI device descriptors.
    pub pci_devices: alloc::vec::Vec<crate::pci::PciDevice>,
    /// PCI command register value.
    pub pci_command: u32,
    /// Memory registers for Realtek network controller.
    pub rtl8139_registers: [u8; 256],
    /// Configured MAC address for network interface.
    pub rtl8139_mac: [u8; 6],
}

#[cfg(test)]
lazy_static! {
    /// Shared software register bus for hardware controller testing.
    pub static ref REGISTER_BUS: Mutex<HardwareRegisterBus> = Mutex::new(HardwareRegisterBus {
        pci_address: 0,
        pci_devices: alloc::vec::Vec::new(),
        pci_command: 0,
        rtl8139_registers: [0; 256],
        rtl8139_mac: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
    });
}

#[cfg(test)]
/// Registers PCI devices in the software bus.
pub fn register_pci_devices(devices: alloc::vec::Vec<crate::pci::PciDevice>) {
    REGISTER_BUS.lock().pci_devices = devices;
}

#[cfg(test)]
/// Resets software bus registers to initial default state.
pub fn reset_hardware_bus() {
    let mut bus = REGISTER_BUS.lock();
    bus.pci_address = 0;
    bus.pci_devices.clear();
    bus.pci_command = 0;
    bus.rtl8139_registers = [0; 256];
    bus.rtl8139_mac = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56];
}

/// Generic wrapper over x86 hardware I/O port operations.
#[derive(Debug, Clone, Copy)]
pub struct IoPort<T> {
    port: u16,
    _phantom: PhantomData<T>,
}

impl<T> IoPort<T> {
    /// Creates a new I/O port instance for target port address.
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            _phantom: PhantomData,
        }
    }
}

impl IoPort<u8> {
    /// Reads an 8-bit byte from the port address.
    pub unsafe fn read(&mut self) -> u8 { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u8>::new(self.port).read()
        }
        #[cfg(test)]
        {
            let bus = REGISTER_BUS.lock();
            if self.port >= 0xC000 && self.port < 0xC006 {
                bus.rtl8139_mac[(self.port - 0xC000) as usize]
            } else if self.port >= 0xC000 && self.port < 0xC100 {
                bus.rtl8139_registers[(self.port - 0xC000) as usize]
            } else {
                0
            }
        }
    }

    /// Writes an 8-bit byte to the port address.
    pub unsafe fn write(&mut self, value: u8) { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u8>::new(self.port).write(value)
        }
        #[cfg(test)]
        {
            let mut bus = REGISTER_BUS.lock();
            if self.port >= 0xC000 && self.port < 0xC100 {
                let reg_offset = (self.port - 0xC000) as usize;
                if reg_offset == 0x37 {
                    // Command register: reset bit 0x10 clears immediately on hardware
                    bus.rtl8139_registers[0x37] = value & !0x10;
                } else {
                    bus.rtl8139_registers[reg_offset] = value;
                }
            }
        }
    }
}

impl IoPort<u16> {
    /// Reads a 16-bit word from the port address.
    pub unsafe fn read(&mut self) -> u16 { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u16>::new(self.port).read()
        }
        #[cfg(test)]
        {
            let bus = REGISTER_BUS.lock();
            if self.port >= 0xC000 && self.port + 1 < 0xC100 {
                let off = (self.port - 0xC000) as usize;
                (bus.rtl8139_registers[off] as u16) | ((bus.rtl8139_registers[off + 1] as u16) << 8)
            } else {
                0
            }
        }
    }

    /// Writes a 16-bit word to the port address.
    pub unsafe fn write(&mut self, value: u16) { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u16>::new(self.port).write(value)
        }
        #[cfg(test)]
        {
            let mut bus = REGISTER_BUS.lock();
            if self.port >= 0xC000 && self.port + 1 < 0xC100 {
                let off = (self.port - 0xC000) as usize;
                bus.rtl8139_registers[off] = (value & 0xFF) as u8;
                bus.rtl8139_registers[off + 1] = ((value >> 8) & 0xFF) as u8;
            }
        }
    }
}

impl IoPort<u32> {
    /// Reads a 32-bit double word from the port address.
    pub unsafe fn read(&mut self) -> u32 { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u32>::new(self.port).read()
        }
        #[cfg(test)]
        {
            let bus = REGISTER_BUS.lock();
            if self.port == 0xCFC {
                // PCI Configuration Data Port
                let addr = bus.pci_address;
                let b = ((addr >> 16) & 0xFF) as u8;
                let s = ((addr >> 11) & 0x1F) as u8;
                let f = ((addr >> 8) & 0x07) as u8;
                let off = (addr & 0xFC) as u8;

                if let Some(dev) = bus.pci_devices.iter().find(|d| d.bus == b && d.slot == s && d.function == f) {
                    match off {
                        0x00 => (dev.device_id as u32) << 16 | (dev.vendor_id as u32),
                        0x04 => bus.pci_command,
                        0x08 => ((dev.class as u32) << 24) | ((dev.subclass as u32) << 16),
                        0x10 => 0xC001, // Base Address Register 0 (I/O base 0xC000, bit 0 set for I/O space)
                        _ => 0,
                    }
                } else {
                    0xFFFF_FFFF
                }
            } else if self.port >= 0xC000 && self.port + 3 < 0xC100 {
                let off = (self.port - 0xC000) as usize;
                let b0 = bus.rtl8139_registers[off] as u32;
                let b1 = (bus.rtl8139_registers[off + 1] as u32) << 8;
                let b2 = (bus.rtl8139_registers[off + 2] as u32) << 16;
                let b3 = (bus.rtl8139_registers[off + 3] as u32) << 24;
                b0 | b1 | b2 | b3
            } else {
                0
            }
        }
    }

    /// Writes a 32-bit double word to the port address.
    pub unsafe fn write(&mut self, value: u32) { // SAFETY: Verified invariant.
        #[cfg(not(test))]
        {
            x86_64::instructions::port::Port::<u32>::new(self.port).write(value)
        }
        #[cfg(test)]
        {
            let mut bus = REGISTER_BUS.lock();
            if self.port == 0xCF8 {
                // PCI Configuration Address Port
                bus.pci_address = value;
            } else if self.port == 0xCFC {
                // PCI Configuration Data Port
                let addr = bus.pci_address;
                let off = (addr & 0xFC) as u8;
                if off == 0x04 {
                    bus.pci_command = value;
                }
            } else if self.port >= 0xC000 && self.port + 3 < 0xC100 {
                let off = (self.port - 0xC000) as usize;
                bus.rtl8139_registers[off] = (value & 0xFF) as u8;
                bus.rtl8139_registers[off + 1] = ((value >> 8) & 0xFF) as u8;
                bus.rtl8139_registers[off + 2] = ((value >> 16) & 0xFF) as u8;
                bus.rtl8139_registers[off + 3] = ((value >> 24) & 0xFF) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies 8-bit, 16-bit, and 32-bit port register roundtrips and unmapped port fallbacks.
    #[test]
    fn test_io_port_widths_and_unmapped_reads() {
        reset_hardware_bus();
        // SAFETY: Under `#[cfg(test)]`, `IoPort` operates purely against the in-memory `REGISTER_BUS`.
        unsafe {
            let mut unmapped_u8 = IoPort::<u8>::new(0x0080);
            assert_eq!(unmapped_u8.read(), 0);
            unmapped_u8.write(0xAA);

            let mut rtl_u16 = IoPort::<u16>::new(0xC040);
            rtl_u16.write(0xBEEF);
            assert_eq!(rtl_u16.read(), 0xBEEF);

            let mut unmapped_u16 = IoPort::<u16>::new(0x0080);
            assert_eq!(unmapped_u16.read(), 0);
            unmapped_u16.write(0x1234);

            let mut rtl_u32 = IoPort::<u32>::new(0xC050);
            rtl_u32.write(0xDEAD_BEEF);
            assert_eq!(rtl_u32.read(), 0xDEAD_BEEF);

            let mut unmapped_u32 = IoPort::<u32>::new(0x0080);
            assert_eq!(unmapped_u32.read(), 0);
            unmapped_u32.write(0x1122_3344);

            let dev = crate::pci::PciDevice {
                bus: 0,
                slot: 1,
                function: 0,
                vendor_id: 0x10EC,
                device_id: 0x8139,
                class: 0x02,
                subclass: 0x00,
            };
            register_pci_devices(alloc::vec![dev]);
            let mut cfg_addr = IoPort::<u32>::new(0xCF8);
            let mut cfg_data = IoPort::<u32>::new(0xCFC);
            cfg_addr.write((1u32 << 31) | (1u32 << 11) | 0x3C);
            assert_eq!(cfg_data.read(), 0);
        }
        reset_hardware_bus();
    }
}
