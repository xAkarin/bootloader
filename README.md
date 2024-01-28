# List from OSDEV (things I need to do)
- Setup 16-bit segment registers and stack [x]
- Print startup message [x]
- Check presence of PCI, CPUID, MSRs [ ]
- Enable and confirm enabled A20 line [ ]
- Load GDTR [ ]
- Inform BIOS of target processor mode [ ]
- Get memory map from BIOS [ ]
- Locate kernel in filesystem [ ]
- Allocate memory to load kernel image [ ]
- Load kernel image into buffer [ ]
- Enable graphics mode [ ]
- Check kernel image ELF headers [ ]
- Enable long mode, if 64-bit [ ]
- Allocate and map memory for kernel segments [ ]
- Setup stack (I assume protected mode stack) [ ]
- Setup COM serial output port [ ]
- Setup IDT [ ]
- Disable PIC [ ]
- Check presence of CPU features (NX, SMEP, x87, PCID, global pages, TCE, WP, MMX, SSE, SYSCALL), and enable them [ ]
- Assign a PAT to write combining [ ]
- Setup FS/GS base [ ]
- Load IDTR [ ]
- Enable APIC and setup using information in ACPI tables [ ]
- Setup GDT and TSS [ ]

# Reasources: 

1. https://stackoverflow.com/questions/33603842/how-to-make-the-kernel-for-my-bootloader
2. https://wiki.osdev.org/Rolling_Your_Own_Bootloader
3. https://stackoverflow.com/questions/57237499/how-to-properly-setup-ss-bp-and-sp-in-x86-real-mode
4. https://thestarman.pcministry.com/asm/debug/Segments.html
5. https://www.cs.cmu.edu/~410-s07/p4/p4-boot.pdf
6. https://www.cs.princeton.edu/courses/archive/fall09/cos318/precepts/precept0
7. https://wiki.osdev.org/Real_Mode#Switching_from_Protected_Mode_to_Real_Mode
8. https://github.com/cfenollosa/os-tutorial/issues/269
9. https://stackoverflow.com/questions/1395591/what-exactly-is-the-base-pointer-and-stack-pointer-to-what-do-they-point
10. https://stackoverflow.com/questions/56008686/printing-a-string-in-16-bit-real-mode
11. https://stackoverflow.com/questions/39281414/why-does-my-bootloader-crash-after-adding-this-line\
12. https://stackoverflow.com/questions/14060411/x86-where-stack-pointer-points
13. https://stackoverflow.com/questions/7279311/difference-between-bx-and-bp
14. https://en.wikibooks.org/wiki/X86_Assembly/16,_32,_and_64_Bits
15. CLD: https://en.wikipedia.org/wiki/Direction_flag
16. https://github.com/klange/toaruos/blob/master/boot/boot.S
17.  POSSIBLE HOLY GRAIL OF INFORMATION BRIGHTSHARD READ THIS: **https://independent-software.com/operating-system-development.html**



