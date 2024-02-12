#![no_std]
#![no_main]
#![allow(unused, dead_code)]


#[no_mangle]
pub extern "C" fn _start(disk_number: u16, partition_table_start: *const u8) -> ! {
    start(disk_number, partition_table_start)
}
#[inline(never)]
fn start(disk_number: u16, partition_table_start: *const u8) -> ! {
    // for i in 0..80 {
    //     for j in 0..25 {
    //         write_str(" ", i*2+j*160);
    //     }
    // }
    write_str("Successfully jumped to stage 2...", 160);
    write_str("Successfully jumped to stage 2...", 160*2);
    write_str("Successfully jumped to stage 2...", 160*3);
    write_str("Successfully jumped to stage 2...", 160*4);
    write_str("Successfully jumped to stage 2...", 160*5);
    // write_str("Successfully jumped to stage 2...", 160*6);
    // write_str("Successfully jumped to stage 2...", 160*7);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    // write_str("Successfully jumped to stage 2...", 0);
    loop {}
}
fn write_str(s: &str, offset: usize) {
    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25 * 2) };
    for (i, char) in s.chars().enumerate() {
        vga_buffer[(i * 2)+offset] = char as u8;
    }
}


#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}