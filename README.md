# Introduction
This is an x86_64 experimental bootloader written in rust (+60 lines of asm).
It will be used by [GluOS](https://github.com/Sxmourai/GluOS).

# Features
Currently it doesn't even support booting, but we will implement:

# Work in progress (TODO in boot order)
## Real mode
- Setup 16-bit segment registers and stack
- Setup stack
- Get RAM size
- Get available video modes
- Get memory map from BIOS
- Enable graphics mode (let the os decide what mode he wants)
- Setup COM serial output port (What is COM ?)

- Setup GDT & TSS, disable interrupts -> Enter protected mode ([How to enter protected mode](https://wiki.osdev.org/Rolling_Your_Own_Bootloader#Ready._Entering_Protected_Mode_...))
## Protected mode
- Enable and confirm A20 line (maybe using Ps/2)
- Check presence of PCI, CPUID, MSRs

- Enable long mode, if 64-bit ([How to enter long mode](https://wiki.osdev.org/Setting_Up_Long_Mode))
## Long mode
- We should just jump to kernel ?

## Other (don't know if it should go in above)
- Setup IDT
- Parse ACPI tables (MADT, FADT, see GluOS -> src/drivers/acpi/tables)
- Enable APIC and setup using information in ACPI tables
- Check presence of CPU features (NX, SMEP, x87, PCID, global pages, TCE, WP, MMX, SSE, SYSCALL), and enable them
- Assign a PAT to write combining (?? Smth about paging)
- Setup FS/GS base ([SwapGS -> FS&GS](https://wiki.osdev.org/SWAPGS#FS_and_GS) and [x86 Registers](https://wiki.osdev.org/CPU_Registers_x86-64#FS.base.2C_GS.base))
- Setup PS/2 controller

