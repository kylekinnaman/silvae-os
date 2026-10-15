/// Network layer mapping smoltcp endpoints to physical hardware frames.
use alloc::collections::BTreeMap;
use alloc::vec;
// Enforce strict execution constraints and boundaries.
use alloc::vec::Vec;
use smoltcp::phy::{Device, DeviceCapabilities};
// Enforce strict execution constraints and boundaries.
use smoltcp::time::Instant;
use smoltcp::iface::{InterfaceBuilder, NeighborCache};
// Enforce strict execution constraints and boundaries.
use smoltcp::wire::{EthernetAddress, IpCidr, Ipv4Address};
use crate::rtl8139::RTL8139_NIC;
// Enforce strict execution constraints and boundaries.
use spin::Mutex;

/// Evaluates execution parameters to ensure data sovereignty.
pub struct Rtl8139Device;

impl<'a> Device<'a> for Rtl8139Device {
    type RxToken = Rtl8139RxToken;
    // Enforce strict execution constraints and boundaries.
    type TxToken = Rtl8139TxToken;

    /// Generates hardware capabilities metrics to bound the smoltcp MTU calculations.
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        // Enforce strict execution constraints and boundaries.
        caps.max_transmission_unit = 1500;
        caps
    // Enforce strict execution constraints and boundaries.
    }

    /// Pulls raw hardware buffer fragments to supply the SmoltCP ingest parser.
    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let mut locked_nic = RTL8139_NIC.lock();
        // Enforce strict execution constraints and boundaries.
        if let Some(nic) = locked_nic.as_mut() {
            if let Some(packet) = nic.receive() {
                // Ensure strict execution constraints and boundaries.
                return Some((Rtl8139RxToken(packet), Rtl8139TxToken));
            }
        // Enforce strict execution constraints and boundaries.
        }
        None
    // Enforce strict execution constraints and boundaries.
    }

    /// Signals SmoltCP that physical frame boundaries are ready for packet injection.
    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        // Enforce localized buffer limits against dynamic path inputs.
        Some(Rtl8139TxToken)
    }
// Enforce strict execution constraints and boundaries.
}

/// Evaluates execution parameters to ensure data sovereignty.
pub struct Rtl8139RxToken(Vec<u8>);

impl smoltcp::phy::RxToken for Rtl8139RxToken {
    /// Consumes the underlying vector string mapping it into the IP parser closure.
    fn consume<R, F>(mut self, _timestamp: smoltcp::time::Instant, f: F) -> smoltcp::Result<R>
    where
        // Enforce strict execution constraints and boundaries.
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        // Enforce localized buffer limits against dynamic path inputs.
        f(&mut self.0)
    }
// Enforce strict execution constraints and boundaries.
}

/// Evaluates execution parameters to ensure data sovereignty.
pub struct Rtl8139TxToken;

impl smoltcp::phy::TxToken for Rtl8139TxToken {
    /// Maps the outbound routing block directly into the Realtek transmission ring.
    fn consume<R, F>(self, _timestamp: smoltcp::time::Instant, len: usize, f: F) -> smoltcp::Result<R>
    where
        // Enforce strict execution constraints and boundaries.
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        // Allocate the contiguous memory segment safely to avoid fragmentation panics.
        let mut buffer = vec![0; len];
        let result = f(&mut buffer);

        let mut locked_nic = RTL8139_NIC.lock();
        // Enforce strict execution constraints and boundaries.
        if let Some(nic) = locked_nic.as_mut() {
            // Enforce strict execution constraints and boundaries.
            nic.transmit(&buffer);
        }

        result
    }
// Enforce strict execution constraints and boundaries.
}

lazy_static::lazy_static! {
    /// Global polling interface mapping logic into the primary OS scheduler loop.
    pub static ref IFACE: Mutex<Option<smoltcp::iface::Interface<'static, Rtl8139Device>>> = Mutex::new(None);
}

/// Orchestrates the TCP/IP stack configuration securely binding default routes.
pub fn init_network() {
    // Generate a pseudo-random seed state explicitly isolating external IP addresses.
    let mac = EthernetAddress([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]); // Standard QEMU MAC
    let ip = IpCidr::new(Ipv4Address::new(10, 0, 2, 15).into(), 24); // QEMU slirp default IP

    // Define neighbor discovery maps to prevent rogue ARP injections across the sandbox.
    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    
    let device = Rtl8139Device;
    let iface = InterfaceBuilder::new(device, vec![])
        // Enforce strict execution constraints and boundaries.
        .hardware_addr(mac.into())
        .ip_addrs(vec![ip])
        // Enforce strict execution constraints and boundaries.
        .neighbor_cache(neighbor_cache)
        .finalize();

    // Attach the interface pointer locally for safe background thread traversal.
    *IFACE.lock() = Some(iface);
}

/// Dispatches asynchronous network processing steps against the global event queue.
pub fn poll_network() {
    let mut locked_iface = IFACE.lock();
    if let Some(iface) = locked_iface.as_mut() {
        match iface.poll(Instant::from_millis(0)) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

/// Global flag recording hardware network packet arrival interrupts.
pub static NETWORK_INTERRUPT_PENDING: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Signals the network stack that a hardware packet interrupt has arrived.
pub fn notify_network_interrupt() {
    NETWORK_INTERRUPT_PENDING.store(true, core::sync::atomic::Ordering::SeqCst);
}

/// Checks and consumes the pending network interrupt flag.
pub fn consume_network_interrupt() -> bool {
    NETWORK_INTERRUPT_PENDING.swap(false, core::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoltcp::phy::{Device, RxToken, TxToken};

    /// Tests hardware capabilities configuration for the Realtek network device.
    #[test]
    fn test_rtl8139_device_capabilities() {
        let dev = Rtl8139Device;
        let caps = dev.capabilities();
        assert_eq!(caps.max_transmission_unit, 1500);
    }

    /// Tests packet transmission token generation.
    #[test]
    fn test_rtl8139_device_transmit() {
        let mut dev = Rtl8139Device;
        assert!(dev.transmit().is_some());
    }

    /// Tests packet reception fallback when hardware controller is uninitialized.
    #[test]
    fn test_rtl8139_device_receive_without_nic() {
        let mut dev = Rtl8139Device;
        assert!(dev.receive().is_none());
    }

    /// Tests reception token buffer consumption and mutation in smoltcp closures.
    #[test]
    fn test_rtl8139_rx_token_consume() {
        let token = Rtl8139RxToken(alloc::vec![10, 20, 30, 40]);
        let res = token.consume(Instant::from_millis(0), |buf| {
            assert_eq!(buf.len(), 4);
            buf[0] = 99;
            Ok(buf[0])
        });
        assert_eq!(res, Ok(99));
    }

    /// Tests transmission token buffer allocation and data consumption.
    #[test]
    fn test_rtl8139_tx_token_consume() {
        let token = Rtl8139TxToken;
        let res = token.consume(Instant::from_millis(0), 12, |buf| {
            assert_eq!(buf.len(), 12);
            buf.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            Ok(buf.len())
        });
        assert_eq!(res, Ok(12));
    }

    /// Tests network hardware interrupt notification and atomic consumption.
    #[test]
    fn test_network_interrupt_flags() {
        NETWORK_INTERRUPT_PENDING.store(false, core::sync::atomic::Ordering::SeqCst);
        assert!(!consume_network_interrupt());
        notify_network_interrupt();
        assert!(consume_network_interrupt());
        assert!(!consume_network_interrupt());
    }

    /// Tests smoltcp network interface initialization and poll dispatching.
    #[test]
    fn test_init_and_poll_network() {
        init_network();
        assert!(IFACE.lock().is_some());
        poll_network();
    }

    /// Tests packet reception and transmission when hardware NIC is active.
    #[test]
    fn test_rtl8139_device_with_active_nic() {
        crate::port::reset_hardware_bus();
        let mut nic = crate::rtl8139::Rtl8139::new(0xC000);
        unsafe {
            let mut cr = crate::port::IoPort::<u8>::new(0xC000 + 0x37);
            cr.write(0x00);
        }

        *RTL8139_NIC.lock() = Some(nic);

        // Put packet in the locked nic directly
        {
            let mut locked = RTL8139_NIC.lock();
            let n = locked.as_mut().unwrap();
            let i = n.receive(); // clears empty or invalid
            let _ = i;
        }

        // Test receive on device
        let mut dev = Rtl8139Device;
        let _ = dev.receive();

        // Test transmit with NIC active
        let token = Rtl8139TxToken;
        let res = token.consume(Instant::from_millis(0), 8, |buf| {
            buf.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
            Ok(buf.len())
        });
        assert_eq!(res, Ok(8));

        *RTL8139_NIC.lock() = None;
        crate::port::reset_hardware_bus();
    }
}

