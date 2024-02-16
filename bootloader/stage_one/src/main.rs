#![no_std]
#![no_main]
#![cfg_attr(debug_assertions, allow(unused, dead_code))]

use core::arch::{asm, global_asm};

global_asm!{r#"
    /*
     * Boiler plate assembly at the start of the program to ensure it jumps to the correct function
     * */
    .code16
    .global __asm_first_stage_entry
    .section .stage1_asm, "awx"
        
    __asm_first_stage_entry:
        /*
         * Sanity check ^^
         * */
        movb $0x0e, %ah
        movb $'m', %al
        int $0x10
        call stage_one_main

     "#, 
    options(att_syntax)
}

#[no_mangle]
extern "C" fn stage_one_main(){
    // loop { 
        unsafe{ 
            asm!("mov ah, 0x0e",
                 "mov al, \'d\'",
                 "int 0x10"
                );
        }
    // }

}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
