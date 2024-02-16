#![no_std]
#![no_main]
// Useless because in release mode
// #![cfg_attr(debug_assertions, allow(unused, dead_code))]
#![allow(unused, dead_code)]

mod disk;
use disk::*;

use core::arch::{asm, global_asm};

global_asm! {r#"
    .section .asm, "awx"
    .global __asm_entry
    .code16

    __asm_entry:
    /*
     * Here we zero all of the segment registers so that their value is known to us. 
     * This is important as we may need to do some memory addressing with the segments and
     * if we do not know their value we cannot accurately memory address!
     * */
        xor ax, ax
        mov ds, ax
        mov es, ax
        mov ss, ax
        mov fs, ax
        mov gs, ax

    /*
     * Here we set the stack pointer to the where our bootstrap was loaded in at. 
     * This can cause some confusion as it is a common misconception that the stack grows upwards,
     * this is not the case. The stack grows downwards, so by setting our stack pointer here
     * we have large enough room for a stack underneath our bootloader
     * */
        mov sp, 0x7c00

    /*
     * Move the base pointer to the same memory address the stack pointer is at. TODO: unsure if
     * this is needed
     * */
        mov bp, sp

    /*
     * Clear the direction flag so we are moving forward in memory when dealing with strings!
     * */
        cld

    enable_a20:
        # enable A20-Line via IO-Port 92, might not work on all motherboards
        in al, 0x92
        test al, 2
        jnz enable_a20_after
        or al, 2
        and al, 0xFE
        out 0x92, al
    enable_a20_after:


    __check_13h_extensions:
        mov ah, 0x41
        mov bx, 0x55aa
        int 0x13
        jc spin 

    __asm_rust_entry:
        push dx
        call bootstrap_main

    disk_error:
        mov ah, 0x0e
        mov al, 'd'
        int 0x10


    spin:
        mov ah, 0x0e
        mov al, 's'
        int 0x10
        hlt
        hlt
        jmp spin
    "#,
}

#[no_mangle]
extern "C" fn bootstrap_main(disk_number: u16) {
    let load_addr = 0x7e00;
    let start_lba = 1;
    let sectors = 1;
    let dap = DiskAddressPacket::from_lba(
        start_lba,
        sectors,
        (load_addr & 0b1111) as u16,
        (load_addr >> 4) as u16,
    );
    unsafe {
        dap.perform_load(disk_number);
    }
    // read_disk(disk_number, LBAReadPacket::new(1, 0, load_address));
    print("Bootstrapper...\r\n"); // QEMU seems to need \r + \n to do a proper new line ?
    unsafe { core::arch::asm!("jmp {:e}", in(reg) load_addr) }
    loop {}
}

#[inline(always)]
fn chr_print(chr: u8) {
    unsafe {
        asm!("mov ah, 0x0e",
         "mov al, {}",
         "int 0x10", in(reg_byte) chr
        );
    }
}
fn print(s: &str) {
    for c in s.chars() {
        chr_print(c as u8)
    }
}

// cfg not test gets rid of an error
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    print("PANIC!");
    loop {}
}
