/// Direct Memory Access (DMA) allocations and circular ring buffer abstractions.
use core::sync::atomic::{AtomicUsize, Ordering};

/// Standard cache-line alignment boundary for hardware DMA operations.
pub const DMA_ALIGNMENT: usize = 64;

/// Statically aligned physical memory block for DMA operations.
#[repr(C, align(64))]
#[derive(Debug)]
pub struct DmaBuffer<const SIZE: usize> {
    /// Raw underlying byte storage with strict 64-byte alignment.
    data: [u8; SIZE],
}

impl<const SIZE: usize> DmaBuffer<SIZE> {
    /// Constructs a zero-initialized DMA buffer.
    pub const fn new() -> Self {
        Self { data: [0; SIZE] }
    }

    /// Returns the physical or identity-mapped memory address for the DMA controller.
    pub fn physical_address(&self) -> u32 {
        self.data.as_ptr() as usize as u32
    }

    /// Returns the byte capacity of the buffer.
    pub const fn capacity(&self) -> usize {
        SIZE
    }

    /// Returns an immutable reference to the raw byte storage.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Returns a mutable reference to the raw byte storage.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Reads a 16-bit word at a byte offset within the DMA buffer.
    pub fn read_u16(&self, offset: usize) -> Option<u16> {
        if offset + 2 <= SIZE {
            let b0 = self.data[offset] as u16;
            let b1 = self.data[offset + 1] as u16;
            Some(b0 | (b1 << 8))
        } else {
            None
        }
    }

    /// Writes a slice of bytes into the DMA buffer at the specified offset.
    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> Result<(), ()> {
        if offset + bytes.len() <= SIZE {
            self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Circular DMA ring buffer controller for packet processing.
#[derive(Debug)]
pub struct DmaRingBuffer<const CAPACITY: usize> {
    /// Underlying DMA buffer with physical memory alignment.
    buffer: DmaBuffer<CAPACITY>,
    /// Current read index (tail pointer) tracked by software.
    read_index: AtomicUsize,
    /// Total number of packets consumed from the ring.
    packets_consumed: AtomicUsize,
}

/// Represents a validated network packet extracted from a DMA ring buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DmaPacket {
    /// Hardware status flags accompanying the packet header.
    pub status: u16,
    /// Length of the packet payload in bytes.
    pub length: usize,
    /// Vector or slice holding the raw packet data.
    pub data: alloc::vec::Vec<u8>,
}

impl<const CAPACITY: usize> DmaRingBuffer<CAPACITY> {
    /// Constructs a new circular DMA ring buffer with zeroed indices.
    pub const fn new() -> Self {
        Self {
            buffer: DmaBuffer::new(),
            read_index: AtomicUsize::new(0),
            packets_consumed: AtomicUsize::new(0),
        }
    }

    /// Returns the physical base address to configure in the hardware NIC register.
    pub fn physical_address(&self) -> u32 {
        self.buffer.physical_address()
    }

    /// Returns the current read offset within the circular ring.
    pub fn read_offset(&self) -> usize {
        self.read_index.load(Ordering::Relaxed)
    }

    /// Returns the total number of packets successfully read.
    pub fn packets_read_count(&self) -> usize {
        self.packets_consumed.load(Ordering::Relaxed)
    }

    /// Writes simulated hardware incoming packet data directly to the ring for testing.
    pub fn write_hardware_packet(&mut self, offset: usize, status: u16, payload: &[u8]) -> Result<usize, ()> {
        let total_len = 4 + payload.len(); // 4 bytes header (status + length)
        if offset + total_len > CAPACITY {
            return Err(());
        }

        let len_bytes = (payload.len() as u16 + 4).to_le_bytes();
        let status_bytes = status.to_le_bytes();

        self.buffer.write_bytes(offset, &status_bytes)?;
        self.buffer.write_bytes(offset + 2, &len_bytes)?;
        self.buffer.write_bytes(offset + 4, payload)?;

        // Return next aligned offset
        let next_offset = (offset + total_len + 3) & !3;
        Ok(next_offset % CAPACITY)
    }

    /// Reads the next available packet from the circular DMA ring.
    pub fn read_packet(&mut self) -> Option<DmaPacket> {
        let current_offset = self.read_index.load(Ordering::Relaxed);
        let status = self.buffer.read_u16(current_offset)?;
        let length_word = self.buffer.read_u16(current_offset + 2)?;

        let total_packet_len = length_word as usize;
        if total_packet_len < 4 || total_packet_len > 1536 {
            return None;
        }

        let payload_len = total_packet_len - 4;
        let mut data = alloc::vec::Vec::with_capacity(payload_len);

        for i in 0..payload_len {
            let ring_pos = (current_offset + 4 + i) % CAPACITY;
            data.push(self.buffer.data[ring_pos]);
        }

        // Align next index to 4-byte boundary and wrap within ring capacity
        let next_index = ((current_offset + total_packet_len + 3) & !3) % CAPACITY;
        self.read_index.store(next_index, Ordering::SeqCst);
        self.packets_consumed.fetch_add(1, Ordering::Relaxed);

        Some(DmaPacket {
            status,
            length: payload_len,
            data,
        })
    }

    /// Resets the ring buffer pointers to the beginning.
    pub fn reset(&mut self) {
        self.read_index.store(0, Ordering::SeqCst);
        self.packets_consumed.store(0, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_buffer_alignment() {
        let buffer = DmaBuffer::<1024>::new();
        let addr = buffer.physical_address() as usize;
        assert_eq!(addr % DMA_ALIGNMENT, 0, "DMA buffer must be 64-byte aligned");
    }

    #[test]
    fn test_dma_buffer_read_write() {
        let mut buffer = DmaBuffer::<256>::new();
        let payload = [0x12, 0x34, 0x56, 0x78];
        assert!(buffer.write_bytes(10, &payload).is_ok());
        assert_eq!(&buffer.as_slice()[10..14], &payload);
        assert_eq!(buffer.read_u16(10), Some(0x3412));
    }

    #[test]
    fn test_dma_ring_buffer_packet_flow() {
        let mut ring = DmaRingBuffer::<4096>::new();
        assert_eq!(ring.read_offset(), 0);

        let test_payload = b"GET /index.html HTTP/1.1\r\n\r\n";
        let next_offset = ring.write_hardware_packet(0, 0x0001, test_payload).expect("write failed");
        assert!(next_offset > 0);

        let packet = ring.read_packet().expect("packet must be available");
        assert_eq!(packet.status, 0x0001);
        assert_eq!(packet.length, test_payload.len());
        assert_eq!(packet.data.as_slice(), test_payload);
        assert_eq!(ring.packets_read_count(), 1);
        assert_eq!(ring.read_offset(), next_offset);
    }

    #[test]
    fn test_dma_ring_buffer_empty_read() {
        let mut ring = DmaRingBuffer::<2048>::new();
        // A zeroed buffer has length = 0, which should return None
        assert!(ring.read_packet().is_none());
    }

    #[test]
    fn test_dma_ring_buffer_boundary_check() {
        let mut buffer = DmaBuffer::<64>::new();
        let oversized = [0xFF; 70];
        assert!(buffer.write_bytes(0, &oversized).is_err());
    }
}
