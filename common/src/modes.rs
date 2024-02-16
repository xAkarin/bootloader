use lazy_static::lazy_static;
use x86_64::structures::{
    gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    tss::TaskStateSegment,
};

#[inline]
/// Enters protected mode, but doesn't jump to kernel !
pub fn enter_protected_mode() {
    load_gdt();
    x86_64::instructions::interrupts::disable();
    // enable_a20_enabled();
}

/// Checks if a20 line is enabled, if not enables it
// fn enable_a20_enabled() {
//     if !unsafe { a20_enabled_from_protected_mode() } {}
//     x86_64::instructions::interrupts::disable();
// }
// / Checks if a20 line is enabled
// / Following https://wiki.osdev.org/A20#Testing_the_A20_line
// / Basically it reads the bootsector identifier (0xAA & 0x55) at two addresses:
// / 0x7DFE (normal address)
// / 0x7DFE + 1MiB
// / But if A20 line is disabled the memory wraps at 1 MiB so it would return the same value. If it's not the same value then a20 line is enabled
// / # Safety
// / Ensure in protected mode
unsafe fn a20_enabled_from_protected_mode() -> bool {
    //////////////////////////// TODO BROKEN
    // unsafe {
    //     let l = core::ptr::read_volatile(0x7DFE as *const u16);
    //     let h = core::ptr::read_volatile((0x7DFE + 1024 * 1024) as *const u16);
    //     l != h
    // }
    // Already enabled in QEMU so it's not that big of a deal...
    true
}

pub fn load_gdt() {
    PROTECTED_GDT.0.load();
    unsafe {
        use x86_64::instructions::segmentation::Segment;
        // x86_64::registers::segmentation::SS::set_reg(SegmentSelector(0));
        // x86_64::registers::segmentation::DS::set_reg(SegmentSelector(0));
        x86_64::registers::segmentation::CS::set_reg(PROTECTED_GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(PROTECTED_GDT.1.tss_selector);
    }
}

const DOUBLE_FAULT_IST_INDEX: usize = 0;
const STACK_SIZE: usize = 4096 * 1024;
lazy_static! {
    pub static ref PROTECTED_TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = x86_64::VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
            stack_start + STACK_SIZE
        };
        tss
    };
}

lazy_static! {
    pub static ref PROTECTED_GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::user_code_segment());
        gdt.add_entry(Descriptor::user_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&PROTECTED_TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
