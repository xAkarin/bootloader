#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]

use core::arch::{asm, global_asm};

global_asm! {r#"
    /*
     * Boiler plate assembly at the start of the program to ensure it jumps to the correct function
     * Kinda useless but enlightens my anxiety doing bootloader dev'ing
     * TODO Remove the need for this
     * */
    .code32
    .global __asm_second_stage_entry
    .section .stage2_asm, "awx"

    spin:
        jmp spin

    __asm_second_stage_entry:
        jmp stage_two_main
     "#,
}

#[no_mangle]
extern "C" fn stage_two_main() {
    // unsafe {asm!("sti")}
    unsafe {core::ptr::write_volatile(0xb8002 as *mut u8, b'g')}
    unsafe {core::ptr::write_volatile(0xb8003 as *mut u8, b'd')}
    // unsafe {core::ptr::write_volatile(0xb8001 as *mut u8, b'a')}
    // unsafe {core::ptr::write_volatile(0xb8002 as *mut u8, b'l')}

    // print("Stage 2...\r\n");

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

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
