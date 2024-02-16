#![no_std]
#![no_main]
// Useless because in release mode
// #![cfg_attr(debug_assertions, allow(unused, dead_code))]
#![allow(unused, dead_code)]

mod disk;
use disk::*;

use core::arch::{asm, global_asm};

global_asm!{r#"
    .section .asm, "awx"
    .global __asm_entry
    .code16

    __asm_entry:
    /*
     *  This ensures that the code segment is set to zero. 
     *
     *  We aren't sure if the bios jumped to
     *  0x7c00:0x0000 or 0x0000:0x7c00 so this is important.  
     * */
        ljmp $0, $__asm_fix_cs

    __asm_fix_cs: 
    /*
     * Here we zero all of the segment registers so that their value is known to us. 
     * This is important as we may need to do some memory addressing with the segments and
     * if we do not know their value we cannot accurately memory address!
     * */
        xor %ax, %ax
        mov %ax, %ds
        mov %ax, %es
        mov %ax, %ss
        mov %ax, %fs
        mov %ax, %gs

    /*
     * Here we set the stack pointer to the where our bootstrap was loaded in at. 
     * This can cause some confusion as it is a common misconception that the stack grows upwards,
     * this is not the case. The stack grows downwards, so by setting our stack pointer here
     * we have large enough room for a stack underneath our bootloader
     * */
        mov $0x7c00, %sp

    /*
     * Move the base pointer to the same memory address the stack pointer is at. TODO: unsure if
     * this is needed
     * */
        mov %sp, %bp

    /*
     * Clear the direction flag so we are moving forward in memory when dealing with strings!
     * */
        cld

    __check_13h_extensions:
        mov $0x41, %ah
        mov $0x55aa, %bx
        int $0x13
        jc spin 

    __asm_rust_entry:
        push %dx
        call bootstrap_main

    disk_error:
    mov $0x0e, %ah
    mov $'d', %al
    int $0x10


    spin:
    mov $0x0e, %ah
    mov $'s', %al
    int $0x10
    hlt
    hlt
    jmp spin

    "#, 
    options(att_syntax)
}


#[no_mangle]
extern "C" fn bootstrap_main(disk_number: u16) {
    chr_print(disk_number as u8);
    let load_address = 0x7c00 + 512;
    read_disk(disk_number, LBAReadPacket::new(1, 0, load_address));
    print("Working!");
    unsafe {core::arch::asm!("jmp {:e}", in(reg) load_address)}
}

#[inline(always)]
fn chr_print(chr:u8){
    unsafe{
        asm!("mov ah, 0x0e",
             "mov al, {}",
             "int 0x10", in(reg_byte) chr
            );
    }
}
fn print(s: &str){
    for c in s.chars() {
        chr_print(c as u8)
    }
}

// cfg not test gets rid of an error
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    print("PANIC!"); 
    loop {
    }
}
