#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic;

global_asm! {r#"
    .section .asm 
    .code16
    .global __asm_entry

    __asm_entry: 
        ljmp $0, $__asm_start

    __asm_start: 
        mov $0x0e, %ah
        mov $'g', %al 
        int $0x10 

        jmp .

"#,
options(att_syntax)}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    loop {}
}
