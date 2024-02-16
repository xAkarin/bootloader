#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]

use core::arch::{asm, global_asm};

mod protected;
use protected::*;

#[no_mangle]
#[link_section = ".stage1_entry"] 
extern "C" fn stage_one_main() {
    // sanity checking 
    //unsafe { 
    //    asm!("mov ah, 0x0e",
    //     "mov al, 'l'",
    //     "int 0x10");
    //}
    
    print("in stage1\r\n");

    loop {}
}

#[inline(always)]
fn chr_print(chr: u8) {
    unsafe {
        asm!("mov ah, 0x0e",
         "int 0x10", 
         in("al") chr
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
