/// Advanced Programmable Interrupt Controller (APIC) interface.
use core::sync::atomic::{AtomicBool, Ordering};

/// Hardware port for the legacy PIC master control register.
pub const PIC_MASTER_CMD: u16 = 0x20;
/// Hardware port for the legacy PIC slave control register.
pub const PIC_SLAVE_CMD: u16 = 0xA0;
/// Hardware port for the legacy PIC master data register.
pub const PIC_MASTER_DATA: u16 = 0x21;
/// Hardware port for the legacy PIC slave data register.
pub const PIC_SLAVE_DATA: u16 = 0xA1;

/// MSR register index for the Local APIC base address.
pub const IA32_APIC_BASE_MSR: u32 = 0x0000_001B;
/// Bit flag indicating that the Local APIC is globally enabled in the MSR.
pub const IA32_APIC_BASE_ENABLE_BIT: u64 = 1 << 11;
/// Bit flag indicating x2APIC mode support in the MSR.
pub const IA32_APIC_BASE_X2APIC_BIT: u64 = 1 << 10;
/// Bit mask to extract the 4 KiB aligned physical address of the Local APIC.
pub const IA32_APIC_BASE_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// Default physical base memory address for the memory-mapped Local APIC.
pub const DEFAULT_APIC_BASE: u64 = 0xFEE0_0000;
/// Default physical base memory address for the I/O APIC.
pub const DEFAULT_IOAPIC_BASE: u64 = 0xFEC0_0000;

/// Local APIC register offset: Local APIC ID.
pub const APIC_REG_ID: usize = 0x020;
/// Local APIC register offset: Local APIC Version.
pub const APIC_REG_VERSION: usize = 0x030;
/// Local APIC register offset: Task Priority Register (TPR).
pub const APIC_REG_TPR: usize = 0x080;
/// Local APIC register offset: End of Interrupt (EOI).
pub const APIC_REG_EOI: usize = 0x0B0;
/// Local APIC register offset: Spurious Interrupt Vector Register (SVR).
pub const APIC_REG_SVR: usize = 0x0F0;
/// Local APIC register offset: Local Vector Table (LVT) Timer.
pub const APIC_REG_LVT_TIMER: usize = 0x320;
/// Local APIC register offset: LVT LINT0.
pub const APIC_REG_LVT_LINT0: usize = 0x350;
/// Local APIC register offset: LVT LINT1.
pub const APIC_REG_LVT_LINT1: usize = 0x360;
/// Local APIC register offset: LVT Error.
pub const APIC_REG_LVT_ERROR: usize = 0x370;

/// Bit flag in the Spurious Vector Register to enable the Local APIC in software.
pub const APIC_SVR_SOFTWARE_ENABLE: u32 = 1 << 8;
/// Standard interrupt vector allocated for spurious interrupts.
pub const APIC_SPURIOUS_VECTOR: u32 = 0xFF;

/// Tracks whether the Local APIC was successfully initialized.
static APIC_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Describes the detected APIC hardware capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApicInfo {
    /// Indicates whether the standard Local APIC is present on the processor.
    pub has_local_apic: bool,
    /// Indicates whether the x2APIC extended architecture is present.
    pub has_x2apic: bool,
    /// Memory-mapped physical address of the Local APIC registers.
    pub base_address: u64,
    /// Indicates whether the APIC is enabled in the MSR.
    pub is_globally_enabled: bool,
}

impl ApicInfo {
    /// Detects APIC hardware features from CPUID registers and system state.
    pub fn detect_from_features(edx_features: u32, ecx_features: u32, msr_val: u64) -> Self {
        let has_local_apic = (edx_features & (1 << 9)) != 0;
        let has_x2apic = (ecx_features & (1 << 21)) != 0;
        let is_globally_enabled = (msr_val & IA32_APIC_BASE_ENABLE_BIT) != 0;
        let mut base_address = msr_val & IA32_APIC_BASE_ADDR_MASK;
        if base_address == 0 {
            base_address = DEFAULT_APIC_BASE;
        }

        Self {
            has_local_apic,
            has_x2apic,
            base_address,
            is_globally_enabled,
        }
    }
}

/// Controller structure for memory-mapped Local APIC operations.
#[derive(Debug, Clone, Copy)]
pub struct LocalApic {
    /// Physical or virtual mapped address of the Local APIC register block.
    base_address: usize,
}

impl LocalApic {
    /// Constructs a new Local APIC controller instance for a target base address.
    pub const fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    /// Reads a 32-bit register value from the Local APIC register space.
    pub unsafe fn read_reg(&self, offset: usize) -> u32 {
        let ptr = (self.base_address + offset) as *const u32;
        core::ptr::read_volatile(ptr)
    }

    /// Writes a 32-bit value to a Local APIC register.
    pub unsafe fn write_reg(&self, offset: usize, value: u32) {
        let ptr = (self.base_address + offset) as *mut u32;
        core::ptr::write_volatile(ptr, value);
    }

    /// Enables the Local APIC in software and sets the spurious interrupt vector.
    pub unsafe fn enable(&self) {
        let svr_val = APIC_SVR_SOFTWARE_ENABLE | APIC_SPURIOUS_VECTOR;
        self.write_reg(APIC_REG_SVR, svr_val);
        // Clear task priority register to accept all interrupt priorities
        self.write_reg(APIC_REG_TPR, 0);
        APIC_INITIALIZED.store(true, Ordering::SeqCst);
    }

    /// Signals End of Interrupt (EOI) to unblock lower priority hardware interrupts.
    pub unsafe fn send_eoi(&self) {
        self.write_reg(APIC_REG_EOI, 0);
    }

    /// Returns the APIC hardware version number.
    pub unsafe fn version(&self) -> u8 {
        (self.read_reg(APIC_REG_VERSION) & 0xFF) as u8
    }

    /// Returns the Local APIC hardware ID for the executing CPU core.
    pub unsafe fn id(&self) -> u8 {
        ((self.read_reg(APIC_REG_ID) >> 24) & 0xFF) as u8
    }
}

/// Global helper to check if the Local APIC is active.
pub fn is_apic_enabled() -> bool {
    APIC_INITIALIZED.load(Ordering::Relaxed)
}

/// Marks the Local APIC active in system tracking.
pub fn set_apic_enabled(enabled: bool) {
    APIC_INITIALIZED.store(enabled, Ordering::SeqCst);
}

/// Sends End of Interrupt (EOI) to the Local APIC register.
pub fn local_apic_eoi(apic_base: usize) {
    let apic = LocalApic::new(apic_base);
    unsafe {
        apic.send_eoi();
    }
}

/// Disables the legacy 8259 PIC controllers by masking all IRQs.
pub unsafe fn disable_legacy_pic() {
    let mut master_data = x86_64::instructions::port::Port::<u8>::new(PIC_MASTER_DATA);
    let mut slave_data = x86_64::instructions::port::Port::<u8>::new(PIC_SLAVE_DATA);
    master_data.write(0xFF);
    slave_data.write(0xFF);
}

/// I/O APIC Redirection Table Entry structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoApicRedirectionEntry {
    /// Target IDT vector number (32..=255).
    pub vector: u8,
    /// Delivery mode (000 = Fixed, 001 = Lowest Priority, etc.).
    pub delivery_mode: u8,
    /// Destination mode (0 = Physical, 1 = Logical).
    pub destination_mode: bool,
    /// Polarity (0 = Active High, 1 = Active Low).
    pub polarity_low: bool,
    /// Trigger mode (0 = Edge, 1 = Level).
    pub trigger_level: bool,
    /// Mask bit (true = masked / disabled, false = unmasked / enabled).
    pub masked: bool,
    /// Target APIC ID of the destination CPU core.
    pub destination_id: u8,
}

impl IoApicRedirectionEntry {
    /// Creates a standard unmasked edge-triggered redirection entry for an IDT vector.
    pub fn new_standard(vector: u8, dest_apic_id: u8) -> Self {
        Self {
            vector,
            delivery_mode: 0,
            destination_mode: false,
            polarity_low: false,
            trigger_level: false,
            masked: false,
            destination_id: dest_apic_id,
        }
    }

    /// Creates a masked redirection entry.
    pub fn new_masked(vector: u8) -> Self {
        Self {
            vector,
            delivery_mode: 0,
            destination_mode: false,
            polarity_low: false,
            trigger_level: false,
            masked: true,
            destination_id: 0,
        }
    }

    /// Encodes the entry into two 32-bit registers (low and high dwords).
    pub fn encode(&self) -> (u32, u32) {
        let mut low = self.vector as u32;
        low |= ((self.delivery_mode & 0x07) as u32) << 8;
        if self.destination_mode {
            low |= 1 << 11;
        }
        if self.polarity_low {
            low |= 1 << 13;
        }
        if self.trigger_level {
            low |= 1 << 15;
        }
        if self.masked {
            low |= 1 << 16;
        }

        let high = (self.destination_id as u32) << 24;
        (low, high)
    }

    /// Decodes two 32-bit registers into an IoApicRedirectionEntry.
    pub fn decode(low: u32, high: u32) -> Self {
        Self {
            vector: (low & 0xFF) as u8,
            delivery_mode: ((low >> 8) & 0x07) as u8,
            destination_mode: (low & (1 << 11)) != 0,
            polarity_low: (low & (1 << 13)) != 0,
            trigger_level: (low & (1 << 15)) != 0,
            masked: (low & (1 << 16)) != 0,
            destination_id: ((high >> 24) & 0xFF) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_apic_fallback_base_address() {
        let edx_has_apic = 1 << 9;
        let info = ApicInfo::detect_from_features(edx_has_apic, 0, 0);
        assert!(info.has_local_apic);
        assert!(!info.has_x2apic);
        assert!(!info.is_globally_enabled);
        assert_eq!(info.base_address, DEFAULT_APIC_BASE);
    }

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

    #[test]
    fn test_ioapic_masked_entry() {
        let entry = IoApicRedirectionEntry::new_masked(0x20);
        let (low, _) = entry.encode();
        assert!((low & (1 << 16)) != 0);
        assert_eq!((low & 0xFF) as u8, 0x20);
    }

    #[test]
    fn test_apic_flag_state() {
        set_apic_enabled(false);
        assert!(!is_apic_enabled());
        set_apic_enabled(true);
        assert!(is_apic_enabled());
        set_apic_enabled(false);
    }
}
