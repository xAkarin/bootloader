//! This file contains functions used for bios functions, they may not work and aren't properly tested
//! This follows https://wiki.osdev.org/BIOS#Common_functions and other docs for their implementation
//! Also https://en.wikipedia.org/wiki/BIOS_interrupt_call#Interrupt_table has more infos

#[repr(u8)]
pub enum BiosInterrupt {
    /// Executed when Shift-Print screen is pressed, as well as when the BOUND instruction detects a bound failure
    ShiftPrint = 0x5,
    /// This is the real time clock interrupt. It fires 18.2 times/second. The BIOS increments the time-of-day counter during this interrupt.
    RTCInterrupt = 0x8,
    /// This is the Keyboard interrupt. This is generally triggered when a key on a keyboard is pressed.
    KeyboardInterrupt = 0x9,
    Video = 0x10,
    HardwareInfo = 0x11,
    /// https://en.wikipedia.org/wiki/Conventional_memory
    ConventionalRAMSize = 0x12,
    /// https://wiki.osdev.org/ATA_in_x86_RealMode_(BIOS)
    Disk = 0x13,
    Serial = 0x14,
    Misc = 0x15,
    KeyboardServices = 0x16,
    PrinterServices = 0x17,
    /// See wiki page cuz it's long https://en.wikipedia.org/wiki/BIOS_interrupt_call#INT_18h:_execute_BASIC
    NetworkBootORFailORBasicCassette = 0x18,
    /// After POST this interrupt is used by the BIOS to load the operating system.
    /// A program can call this interrupt to reboot the computer
    /// (but must ensure that hardware interrupts or DMA operations will not cause the system to hang or crash during either the reinitialization of the system by BIOS
    /// or the boot process).
    Reboot = 0x19,
    RTCServices = 0x1A,
    // Same as RTC services, but ax/ah is different
    // PCIServices,
    /// Ctrl-Break handler - called by INT 09 when Ctrl-Break has been pressed
    CtrlBreakHandler = 0x1B,
    /// Timer tick handler - called by INT 08
    TimerTickHandler = 0x1C,
    // Others, but not usefull
}
/// Issues a int 0x10 with ah set to ah argument
/// # Safety
/// Must ensure this is a valid bios interrupt, and it won't break stuff
#[inline(always)]
unsafe fn int(int_num: u16, ax: u16) {
    unsafe {
        core::arch::asm!("int {0:x}",in(reg) int_num, in("ax") ax);
        // TODO Return result if carry flag is set
    }
}

/// More info: https://fr.wikipedia.org/wiki/INT_10H
/// TODO Supported everything there ^^
pub mod video {
    use super::*;
    use crate::{read_reg, write_reg};
    use core::ffi::c_char;

    /// # Safety
    /// Ensure the video mode is supported
    /// You can use: https://wiki.osdev.org/VESA_Video_Modes#How_to_pick_the_mode_I_wish.3F
    /// TODO Make a safe wrapper around supported video mode
    /// TODO Return a better result than just a u8 (i.e. flags)
    pub unsafe fn change_video_mode(video_mode: u8) -> u8 {
        unsafe {
            core::arch::asm!("", in("al") video_mode);
            int(0x10, 0);
        }
        crate::read_reg!("al", u8)
    }
    pub fn get_controller_info(return_addr: *mut ()) -> Option<&'static VbeInfoBlock> {
        unsafe { core::arch::asm!("mov es, {0:e}", in(reg) seg(return_addr as usize) as u32) }
        unsafe {
            core::arch::asm!("mov di, {0:e}", in(reg) off(return_addr as usize).unwrap() as u32)
        }
        unsafe { int(0x10, 0x4F00) }
        if read_reg!("ax", u32) == 0x004F {
            None
        } else {
            Some(unsafe { &*(return_addr as *const VbeInfoBlock) })
        }
    }
    pub fn get_mode_info(mode: u16, return_addr: *mut ()) -> Option<&'static VbeModeInfo> {
        // Can't use write_reg! on es di :c
        unsafe { core::arch::asm!("mov es, {0:e}", in(reg) seg(return_addr as usize) as u32) }
        unsafe {
            core::arch::asm!("mov di, {0:e}", in(reg) off(return_addr as usize).unwrap() as u32)
        }
        write_reg!("cx", mode);
        unsafe { int(0x10, 0x4F01) }
        if read_reg!("ax", u32) == 0x004F {
            None
        } else {
            Some(unsafe { &*(return_addr as *const VbeModeInfo) })
        }
    }
    #[repr(packed)]
    pub struct VbeModeInfo {
        ///  deprecated, only bit 7 should be of interest to you, and it indicates the mode supports a linear frame buffer.
        pub attributes: u16,
        /// deprecated
        pub window_a: u8,
        /// deprecated
        pub window_b: u8,
        /// deprecated - used while calculating bank numbers
        pub granularity: u16,
        pub window_size: u16,
        pub segment_a: u16,
        pub segment_b: u16,
        /// deprecated - used to switch banks from protected mode without returning to real mode
        pub win_func_ptr: u32,
        /// number of bytes per horizontal line
        pub pitch: u16,
        /// width in pixels
        pub width: u16,
        /// height in pixels
        pub height: u16,
        /// unused...
        pub w_char: u8,
        pub y_char: u8,
        pub planes: u8,
        /// bits per pixel in this mode
        pub bits_per_pixel: u8,
        /// deprecated - total number of banks in this mode
        pub banks: u8,
        pub memory_model: u8,
        /// deprecated - size of a bank - almost always 64 KB but may be 16 KB...
        pub bank_size: u8,
        pub image_pages: u8,
        pub reserved0: u8,
        pub red_mask: u8,
        pub red_position: u8,
        pub green_mask: u8,
        pub green_position: u8,
        pub blue_mask: u8,
        pub blue_position: u8,
        pub reserved_mask: u8,
        pub reserved_position: u8,
        pub direct_color_attributes: u8,
        /// physical address of the linear frame buffer - write here to draw to the screen
        pub framebuffer: u32,
        pub off_screen_mem_off: u32,
        /// size of memory in the framebuffer but not being displayed on the screen
        pub off_screen_mem_size: u16,
        _reserved1: [u8; 206],
    }
    #[repr(packed)]
    pub struct VbeInfoBlock {
        /// == "VESA"
        pub vbe_sig: [c_char; 4],
        /// == 0x0300 for VBE 3.0
        pub vbe_version: u16,
        /// isa vbeFarPtr
        pub oem_str_ptr: u32,
        pub capas: u32,
        /// isa vbeFarPtr
        pub video_mode_ptr: u32,
        /// as # of 64KB blocks
        pub total_mem: u16,
        _reserved: [u8; 492],
    }
}

/// https://en.wikipedia.org/wiki/INT_13H
/// And https://wiki.osdev.org/ATA_in_x86_RealMode_(BIOS)
pub mod disk {
    use crate::write_reg;

    use super::int;

    #[repr(C, packed)]
    pub struct LBAReadPacket {
        /// Size of the DAP structure
        pub size: u8,
        /// always zero
        _zero: u8,
        /// Number of sectors to transfer
        pub number_of_sectors: u16,
        pub addr: u32,
        /// Start logical block address
        pub start_lba: u64,
    }
    impl LBAReadPacket {
        pub fn new(sectors: u16, addr: u32, lba: u64) -> Self {
            Self {
                size: 0x10,
                _zero: 0,
                number_of_sectors: sectors,
                addr,
                start_lba: lba,
            }
        }
    }
    #[inline]
    /// bit 7 set means reset both hard and floppy disks
    pub fn reset(disk: u8) {
        write_reg!("dl", disk);
        unsafe { int(0x13, 0) }
        // TODO Return result
    }

    /// Returns a pointer to the read sectors, but you can use your pointers passed into LBAReadPacket creation
    #[inline]
    pub fn read_lba(disk_number: u16, packet: LBAReadPacket) -> &'static [u8] {
        let packet_addr = (&packet as *const LBAReadPacket) as u16;
        unsafe {
            core::arch::asm!(
                "push 0x7a", // error code `z`, passed to `fail` on error
                "mov {1:x}, si", // backup the `si` register, whose contents are required by LLVM
                "mov si, {0:x}",
                "int 0x13",
                "pop si", // remove error code again
                "mov si, {1:x}", // restore the `si` register to its prior state
                in(reg) packet_addr,
                out(reg) _,
                in("ax") 0x4200u16,
                in("dx") disk_number,
            );
            core::slice::from_raw_parts(
                packet.addr as *const u8,
                packet.number_of_sectors as usize * 512,
            )
        }
    }
    // Write LBA uses CHS, which I don't wanna use
}

/// https://wiki.osdev.org/Detecting_Memory_(x86)
///
pub mod memory {
    use super::int;
    use crate::{carry_set, read_reg};
    use core::arch::asm;

    /// https://wiki.osdev.org/Detecting_Memory_(x86)#Detecting_Low_Memory
    /// Returns memory size in Kb
    pub fn get_low_memory() -> Option<u32> {
        unsafe { int(0x12, 0) };
        if carry_set() {
            return None;
        }
        Some(read_reg!("ax"))
    }
    // /// https://wiki.osdev.org/Detecting_Memory_(x86)#Detecting_Upper_Memory
    // /// Said to be the best way to detect memory (ultimate way)
    // pub fn get_upper_memory(dst: *const ()) {
    //     // write_reg!("ebx", 0);
    //     let r: u8;
    //     unsafe {
    //         asm!("
    //     mov ebx, 0
    //     mov edx, 0x534D4150
    //     mov ecx, 0x18
    //     mov eax, 0xE820
    //     int 0x15
    //     jc 22
    //     cmp ebx, 0
    //     jeq 23
    //     jmp 21
    //     21: ; If it worked
    //     mov eax, ebx
    //     jmp 2F
    //     23: ; If ebx == 0, so end of map
        // Not done
    //     jmp 2F
    //     22: ; Failed
    //     mov eax, 1
    //     2F:
    //     ", out("eax") r)
    //     };
    //     //TODO Check read_reg!("ebx", u32) != 0 but LLVM uses it so idk how we could do it
    //     if r == 0 && read_reg!("eax", u32) == 0x534D4150 {
    //         //TODO Store ebx for next call to function
    //         let len: u8 = read_reg!("cl");
    //     }
    // }
}

#[inline(always)]
/// Puts carry flag in al
/// Returns true if carry flag is set
pub fn carry_set() -> bool {
    let r: u8;
    unsafe {
        core::arch::asm!("
jc 2a
jmp 2b
2a:
    mov al, 1
    jmp 2c
2b:
    mov al, 0
2c:
        ", out("al") r)
    }
    r == 0
}

#[macro_export]
/// # Safety
/// This is obviously unsafe but it's easier to use without unsafe
macro_rules! read_reg {
    ($register: tt, $size: ty) => {{
        let r: $size;
        unsafe {core::arch::asm!("", out($register) r)};
        r
    }};
    ($register: tt) => {{
        let r;
        unsafe {core::arch::asm!("", out($register) r)};
        r
    }};
}

#[macro_export]
/// # Safety
/// This is obviously unsafe but it's easier to use without unsafe
macro_rules! write_reg {
    ($reg: tt, $value: expr) => {{
        unsafe {core::arch::asm!("", in($reg) $value)};
    }}
    // ($($registers: tt)*) => {{
    //     unsafe {core::arch::asm!("", $($registers)*)};
    // }};
}

/// Gets the segment of an address
pub fn seg(addr: usize) -> u8 {
    (addr & 0b1111) as u8
}
/// Gets the offset of an address (to use with seg)
pub fn off(addr: usize) -> Option<u8> {
    (addr >> 4).try_into().ok()
}
