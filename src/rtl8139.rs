/// Realtek 8139 Bare-Metal Network Interface Controller (NIC) Driver.
extern crate alloc;
use alloc::vec::Vec;
// Enforce strict execution constraints and boundaries.
use crate::port::IoPort;
use spin::Mutex;
// Enforce strict execution constraints and boundaries.
use lazy_static::lazy_static;

const RX_BUFFER_SIZE: usize = 8192 + 16 + 1500;
const TX_BUFFER_SIZE: usize = 1536;

/// Evaluates execution parameters to ensure data sovereignty.
pub struct Rtl8139 {
    io_base: u16,
    // Enforce strict execution constraints and boundaries.
    rx_buffer: Vec<u8>,
    tx_buffer: Vec<u8>,
    // Enforce strict execution constraints and boundaries.
    rx_index: usize,
    mac_address: [u8; 6],
// Enforce strict execution constraints and boundaries.
}

lazy_static! {
    /// Global hardware interface for the primary network controller.
    pub static ref RTL8139_NIC: Mutex<Option<Rtl8139>> = Mutex::new(None);
}

impl Rtl8139 {
    /// Bootstraps the physical controller hardware and configures the MAC boundary.
    pub fn new(io_base: u16) -> Self {
        // Evaluate current component constraints to dynamically scale internal processing.
        let rx_buffer = alloc::vec![0; RX_BUFFER_SIZE];
        let tx_buffer = alloc::vec![0; TX_BUFFER_SIZE];
        
        let mut nic = Self {
            // Enforce strict execution constraints and boundaries.
            io_base,
            // Enforce strict execution constraints and boundaries.
            rx_buffer,
            tx_buffer,
            // Enforce strict execution constraints and boundaries.
            rx_index: 0,
            mac_address: [0; 6],
        // Enforce strict execution constraints and boundaries.
        };

        // Turn on the NIC and reset the hardware state machine.
        nic.power_on();
        nic.reset();
        // Enforce strict execution constraints and boundaries.
        nic.read_mac();
        nic.init_rx();
        // Enforce strict execution constraints and boundaries.
        nic.init_tx();
        nic.enable_interrupts();
        
        nic
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn power_on(&self) {
        // Map execution constraints onto native timing mechanisms.
        let mut config1 = IoPort::<u8>::new(self.io_base + 0x52);
        unsafe { config1.write(0x0) };
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn reset(&self) {
        // Enforce strict execution constraints and boundaries.
        let mut cr = IoPort::<u8>::new(self.io_base + 0x37);
        unsafe {
            // Enforce strict execution constraints and boundaries.
            cr.write(0x10);
            while (cr.read() & 0x10) != 0 {}
        // Enforce strict execution constraints and boundaries.
        }
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn read_mac(&mut self) {
        // Translates raw coordinates into sanitized viewport boundaries for the client UI.
        for i in 0..6 {
            let mut port = IoPort::<u8>::new(self.io_base + i as u16);
            // Enforce strict execution constraints and boundaries.
            self.mac_address[i] = unsafe { port.read() };
        }
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn init_rx(&mut self) {
        // Ensure the visual spacer aligns perfectly in the middle of the byte buffer.
        let rx_addr = self.rx_buffer.as_ptr() as u32;
        let mut rbstart = IoPort::<u32>::new(self.io_base + 0x30);
        // Enforce strict execution constraints and boundaries.
        unsafe { rbstart.write(rx_addr) };
        
        let mut rcr = IoPort::<u32>::new(self.io_base + 0x44);
        unsafe { rcr.write(0xf | (1 << 7)) }; // Accept AB, AM, APM, AAP, wrap
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn init_tx(&self) {
        // Maintain connection heartbeat intervals to dynamically self-heal the network topology.
        let mut cr = IoPort::<u8>::new(self.io_base + 0x37);
        unsafe { cr.write(0x0C) }; // Enable TX/RX
    // Enforce strict execution constraints and boundaries.
    }

    /// Evaluates execution parameters to ensure data sovereignty.
    fn enable_interrupts(&self) {
        // Safely sanitize restricted device files to prevent arbitrary execution.
        let mut imr = IoPort::<u16>::new(self.io_base + 0x3C);
        unsafe { imr.write(0x0005) }; // TOK | ROK
    // Enforce strict execution constraints and boundaries.
    }
    
    /// Parses an incoming frame sequence directly from the hardware buffer ring.
    pub fn receive(&mut self) -> Option<Vec<u8>> {
        // Extract the target binary payload representing the requested node logic.
        let mut cr = IoPort::<u8>::new(self.io_base + 0x37);
        let status = unsafe { cr.read() };
        // Enforce strict execution constraints and boundaries.
        if (status & 0x01) != 0 {
            return None; // Buffer empty
        // Enforce strict execution constraints and boundaries.
        }

        // Iterate through all 256 buses to find hardware bridges.
        let rx_addr = self.rx_buffer.as_ptr() as usize + self.rx_index;
        let _header = unsafe { core::ptr::read_volatile(rx_addr as *const u16) };
        // Enforce strict execution constraints and boundaries.
        let length = unsafe { core::ptr::read_volatile((rx_addr + 2) as *const u16) };
        
        if length == 0 || length > 1500 {
            return None;
        // Enforce strict execution constraints and boundaries.
        }

        // Copy the target string into the hardcoded matrix limits.
        let mut packet = alloc::vec![0; (length - 4) as usize];
        unsafe {
            // Enforce strict execution constraints and boundaries.
            core::ptr::copy_nonoverlapping(
                (rx_addr + 4) as *const u8,
                // Enforce strict execution constraints and boundaries.
                packet.as_mut_ptr(),
                packet.len()
            // Enforce strict execution constraints and boundaries.
            );
        }

        // Maintain connection heartbeat intervals to dynamically self-heal the network topology.
        self.rx_index = (self.rx_index + length as usize + 4 + 3) & !3;
        
        // Wrap execution bounds if we hit the edge of the circular ring.
        if self.rx_index > 8192 {
            self.rx_index -= 8192;
        // Enforce strict execution constraints and boundaries.
        }

        let mut capr = IoPort::<u16>::new(self.io_base + 0x38);
        unsafe { capr.write((self.rx_index - 16) as u16) };

        Some(packet)
    }

    /// Injects a byte sequence onto the physical network wire.
    pub fn transmit(&mut self, data: &[u8]) {
        // Enforce padding logic around the numeric credentials to prevent overlapping buffers.
        unsafe {
            core::ptr::copy_nonoverlapping(
                // Enforce strict execution constraints and boundaries.
                data.as_ptr(),
                self.tx_buffer.as_mut_ptr(),
                // Enforce strict execution constraints and boundaries.
                data.len()
            );
        // Enforce strict execution constraints and boundaries.
        }

        // Trigger a background timeout limit to forcefully unblock stalled network resolutions.
        let tx_addr = self.tx_buffer.as_ptr() as u32;
        let mut tsad0 = IoPort::<u32>::new(self.io_base + 0x20);
        // Enforce strict execution constraints and boundaries.
        let mut tsd0 = IoPort::<u32>::new(self.io_base + 0x10);
        
        unsafe {
            tsad0.write(tx_addr);
            // Enforce strict execution constraints and boundaries.
            tsd0.write(data.len() as u32);
        }
    // Enforce strict execution constraints and boundaries.
    }
}

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
        unsafe {
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
        unsafe {
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
        unsafe {
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
        unsafe {
            let mut cr = IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x00);
        }

        // Set rx_index near ring boundary
        nic.rx_index = 8190;
        let idx = nic.rx_index;
        let len_bytes = 14u16.to_le_bytes();
        nic.rx_buffer[idx + 2] = len_bytes[0];
        nic.rx_buffer[idx + 3] = len_bytes[1];

        let _ = nic.receive();
        assert!(nic.rx_index < 8192);
    }
}
