
#[repr(C, packed)]
pub struct LBAReadPacket {
    /// Size of the DAP structure
    size: u8,
    /// always zero
    _zero: u8,
    /// Number of sectors to transfer
    sectors: u16,
    /// Offset to memory buffer
    offset: u16,
    /// Segment of memory buffer
    segment: u16,
    /// Start logical block address
    lba: u64,
}
impl LBAReadPacket {
    pub fn new(sectors_count: u16, start_sector: u64, output_buffer: u32) -> Self {
        Self {
            size: 0x10,
            _zero: 0,
            sectors: sectors_count,
            offset: (output_buffer & 0b1111) as u16,
            segment: (output_buffer >> 4) as u16,
            lba: start_sector,
        }
    }
}
pub fn read_disk(disk_number: u16, packet: LBAReadPacket) {
    let packet_addr = &packet as *const LBAReadPacket as u16;
    unsafe {
        core::arch::asm!(
            "push 0x7a", // error code `z`, passed to `fail` on error
            "mov {1:x}, si", // backup the `si` register, whose contents are required by LLVM
            "mov si, {0:x}",
            "int 0x13",
            "jc disk_error",
            "pop si", // remove error code again
            "mov si, {1:x}", // restore the `si` register to its prior state
            in(reg) packet_addr,
            out(reg) _,
            in("ax") 0x4200u16,
            in("dx") disk_number,
        );
    }
}