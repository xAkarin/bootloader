#![no_std] 
#![no_main]

//#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn _start() -> ! { 
    loop {}
 }

use::core::panic::PanicInfo;

#[panic_handler]
#[allow(unused_variables)]
fn panic(info: &PanicInfo) -> ! {
     loop {} 
}