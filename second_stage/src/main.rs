#![no_std]
#![no_main]
#![allow(unused, dead_code)]


#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start(disk_number: u16, partition_table_start: *const u8) -> ! {
    start(disk_number, partition_table_start)
}
#[inline(never)]
fn start(disk_number: u16, partition_table_start: *const u8) -> ! {
    write_str("Hello this is a long string that will take more than 512 bytes to check that everything is worrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrking");
    loop {}
}
fn write_str(s: &str) {
    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25 * 2) };
    for (i, char) in s.chars().enumerate() {
        vga_buffer[i * 2] = char as u8;
    }
}


#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}