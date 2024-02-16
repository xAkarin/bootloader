#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]

use core::arch::{asm, global_asm};

global_asm! {r#"
    /*
     * Boiler plate assembly at the start of the program to ensure it jumps to the correct function
     * Kinda useless but enlightens my anxiety doing bootloader dev'ing
     * */
    .code16
    .global __asm_first_stage_entry
    .section .stage1_asm, "awx"
        
    __asm_first_stage_entry:
        jmp stage_one_main
     "#,
}

#[no_mangle]
extern "C" fn stage_one_main() {
    print("Stage 1...\r\n");
    print("Stage 1 lorem...\r\n");
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
