


#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod pci;
pub mod rtl8139;
pub mod net;
pub mod allocator;
pub mod apic;
pub mod dma;
pub mod port;
include!("main__p004.rs");
