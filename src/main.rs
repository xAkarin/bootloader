#![no_std]
#![no_main]

use core::arch::global_asm;
global_asm!(include_str!("boot.s"));

const ENTRY_POINT_ADDR: u64 = 0x7c00;

#[no_mangle]
pub extern "C" fn first_stage(disk_number: u16) {
    // write_str("     ");
    // let partition_table = unsafe { core::slice::from_raw_parts((0x7c00+446) as *const u8, 16 * 4) };
    // write_v8(&[((disk_number as u8 + b'0') as char) as u8], 0);
    let buffer_addr = (ENTRY_POINT_ADDR + 512) as *mut ();
    write_v8(&[b'R'], 2);
    read_disk(disk_number, LBAReadPacket::new(1, buffer_addr, 1).unwrap());
    write_v8(
        unsafe { core::slice::from_raw_parts(buffer_addr as *mut u8, 512) },
        160,
    );
    write_v8(&[b'D'], 4);
    // write_v8(&[second_stage_start() as u8], 6);

    loop {}
}

#[repr(C, packed)]
struct LBAReadPacket {
    /// size of packet, so 16 bytes
    size: u8,
    _zero: u8,
    /// max 127 on some BIOSes
    n_sectors: u16,
    /// 16 bit segment:16 bit offset
    transfer_buffer: u32,
    low_lba: u32,
    /// upper 16-bits of 48-bit starting LBA
    hig_16bit_start_lba: u32,
}
impl LBAReadPacket {
    pub fn new(sectors: u16, buffer_addr: *mut (), lba: u64) -> Option<Self> {
        if lba >= 2 ^ 48 {
            return None;
        }
        let addr = buffer_addr as u32;
        Some(Self {
            size: core::mem::size_of::<Self>() as u8,
            _zero: 0,
            n_sectors: sectors,
            transfer_buffer: addr,
            low_lba: (lba & u32::MAX as u64) as u32,
            hig_16bit_start_lba: (lba >> 32) as u32,
        })
    }
}
/// If the disk drive itself does not support LBA addressing, the BIOS will automatically convert the LBA to a CHS address for you -- so this function still works.
fn read_disk(disk: u16, packet: LBAReadPacket) {
    let packet_addr = (&packet as *const LBAReadPacket) as u16;
    unsafe {
        core::arch::asm!(
            "mov {1:x}, si", // backup the `si` register, whose contents are required by LLVM
            "mov si, {0:x}",
            "int 0x13",
            in(reg) packet_addr,
            out(reg) _,
            in("ax") 0x4200u16,
            in("dx") disk,
        );
    }
}

fn write_str(s: &str) {
    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25 * 2) };
    for (i, char) in s.chars().enumerate() {
        vga_buffer[i * 2] = char as u8;
    }
}
fn write_v8(s: &[u8], offset: usize) {
    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25 * 2) };
    for (i, char) in s.iter().enumerate() {
        vga_buffer[i * 2 + offset] = *char as u8;
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// extern "C" {
//     static _second_stage_start: u8;
// }

// fn second_stage_start() -> *const () {
//     let ptr: *const u8 = unsafe { &_second_stage_start };
//     ptr as *const ()
// }