#![no_std]
#![no_main]
#![allow(unused, dead_code)]


#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start(disk_number: u16, partition_table_start: *const u8) -> ! {
    start(disk_number, partition_table_start)
}

fn start(disk_number: u16, partition_table_start: *const u8) -> ! {
    loop {}
}


#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}