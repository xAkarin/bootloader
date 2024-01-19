.code16
.global _start 
.section .text 

/* This is a form of debugging macro, just used to ensure that the program flow is correct */
.macro SANITY_CHECK
    mov $0x0e, %ah 
    mov $'[', %al 
    int $0x10 
    mov $'O', %al 
    int $0x10 
    mov $'k', %al 
    int $0x10
    mov $']', %al 
    int $0x10
    mov $'\n', %al 
    int $0x10 
    mov $'\r', %al 
    int $0x10 
.endm 

_start:
    /* Ensure that the bootloader is actually running from the correct address */ 
    SANITY_CHECK

    /* This jump will 0 the code segment that way when calling a label the address will be (0 * 16):(+ offset) */
    ljmp $0, $_main

    /* The code jumps over this, this ensures that the ljmp actually occured and we did not fall into _main */
    SANITY_CHECK

_main:
    /* Ensure that it did infact jump to main */ 
    SANITY_CHECK

    /* Here we zero out the segment registers as the values given to us by the bios are unknown */
    xor %ax, %ax 
    mov %ax, %ds 
    mov %ax, %es 
    mov %ax, %ss 

    /* Set the stack pointer to our bootloader address (the stack grows downward from here) */ 
    mov $0x7c00, %sp 

    /* Test that the call instruction works (I had a bug previously where it did not) */
    call _test_call

    mov $_bootloader_hello_world, %si 
    call _print

    jmp .

/* This labels serves the purpose to check that the call instruction properly works */ 
_test_call:
    SANITY_CHECK
    ret

/* Bootloader print message */
_print: 
    /* ensure we are in teletype mode */ 
    mov $0x0e, %ah 
_print_loop: 
    /* Load byte at memory address in %si, into %al then increment %si by 1 byte */
    lodsb 
    /* Compare %al to $0 (null) */
    cmp $0, %al 
    /* If (%al == NULL){ jmp _print_done } */
    je _print_done
    /* else { use 0x10 interupt to print value in al to screen }*/ 
    int $0x10 
    /* Loop */
    jmp _print_loop
_print_done:
    /* Return to the address that was pushed onto the stack by the call instruction */
    ret 

/* Bootloader hello world message to test if the print function is working */
_bootloader_hello_world: 
    .asciz "Bootloader Hello World!\n\r"

/* Pad the binary with bytes until it is 510 bytes in size */
.org 510 
/* Then add the magical MBR word value to the end of the binary making it 512 bytes and bootable */
.word 0xaa55 
