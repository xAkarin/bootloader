#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

global_asm!{r#"
    .section .asm, "awx"
    .global __asm_entry
    .code16

    __asm_entry:
        ljmp $0, $__asm_fix_cs

    __asm_fix_cs: 
        mov $0x0e, %ah
        mov $'g', %al
        int $0x10
        
    spin:
        mov $0x0e, %ah
        mov $'g', %al
        int $0x10
        jmp spin

    "#, 
    options(att_syntax)
}









//const ENTRY_POINT_ADDR: u16 = 0x7c00;
//const SECOND_STAGE_ADDR: u32 = (ENTRY_POINT_ADDR as u32+0x1000); // Adds a bit of padding 
// TODO Adjust sector step.
//const SECTOR_STEP: u16 = 1;
//const MAX_SECTORS: u32 = u16::MAX as u32;

//#[no_mangle]
//pub extern "C" fn first_stage(disk_number: u16) {
//    write_v8(&[b'S', b't', b'a', b'g', b'e', b' ', b'1'], 0);
//  let mut sector_number: u32 = 1;
//   let mut buffer_addr = SECOND_STAGE_ADDR;
//    loop {
//        let n = ((sector_number&0xFF) as u8);
//        write_v8(&[b'B',n+b'0'], 160*n as usize);
//        // write_v8(&[b'0'+n], 160);
//        read_disk(disk_number, LBAReadPacket::new(SECTOR_STEP, buffer_addr, sector_number as u64).unwrap());
//        write_v8(&[b'D'], 160*n as usize+4);
//        let buffer = unsafe{core::slice::from_raw_parts(buffer_addr as *const u16, 256)};
//        if *buffer.last().unwrap() == 0xdead {
//            break;
//        }
//        // write_v8(&[b'D'], 160*n as usize);
//        
//        //TODO Investigate why this line breaks everything ?
//        // write_v8(&[b'0'+sector_number as u8], (sector_number as usize)*160);
//        // write_v8(&[n], 320);
//        buffer_addr += SECTOR_STEP as u32*512;
//        sector_number += SECTOR_STEP as u32;
//    }
//    write_v8(&[b'F'], 0);
//    unsafe{core::arch::asm!("call {addr:e}", addr=in(reg) buffer_addr)};
//    write_v8(&[b'P'], 0);
//    loop {} // Could use unreachable! but it adds the unwrap text, which takes quite a lot of space !
//}

//#[repr(C, packed)]
//struct LBAReadPacket {
    /// where the size of the packet is stored
    /// OSDev is a bit missleading, this is where a number describing the size of the packet is, so 16 bytes
//    size: u8,
    /// Always zero 
//    _zero: u8,
    /// max 127 on some BIOSes
//    n_sectors: u16,
    /// 16 bit segment:16 bit offset
//    transfer_buffer: u32,
//    low_lba: u32,
    /// upper 16-bits of 48-bit starting LBA
//   hig_16bit_start_lba: u32,
//}
//impl LBAReadPacket {
//    pub fn new(sectors: u16, addr: u32, lba: u64) -> Option<Self> {
//        if lba >= 2 ^ 48 {
//            return None;
//        }
//        Some(Self {
//            size: core::mem::size_of::<Self>() as u8,
//            _zero: 0,
//            n_sectors: sectors,
//            transfer_buffer: addr,
//            low_lba: (lba & u32::MAX as u64) as u32,
//            hig_16bit_start_lba: (lba >> 32) as u32,
//        })
//    }
//}
/// If the disk drive itself does not support LBA addressing, the BIOS will automatically convert the LBA to a CHS address for you -- so this function still works.
//fn read_disk(disk_number: u16, packet: LBAReadPacket) {
//    let packet_addr = (&packet as *const LBAReadPacket) as u16;
//    let mut a = 0;
//    unsafe {
//        core::arch::asm!(
//           "push 0x7a", // error code `z`, passed to `fail` on error
//            "mov {1:x}, si", // backup the `si` register, whose contents are required by LLVM
//            "mov si, {0:x}",
//            "int 0x13",
//            "jc spin",
//            "pop si", // remove error code again
//            "mov si, {1:x}", // restore the `si` register to its prior state
//            in(reg) packet_addr,
//            out(reg) a,
//            in("ax") 0x4200u16,
//            in("dx") disk_number,        
//        );
//    }
//}
//fn write_v8(s: &[u8], offset: usize) {
//    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25 * 2) };
//    for (i, char) in s.iter().enumerate() {
//        vga_buffer[i * 2 + offset] = *char as u8;
//        vga_buffer[i * 2 + offset+1] = 0x0F;
//    }
//}

/// cfg not test gets rid of an error
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
