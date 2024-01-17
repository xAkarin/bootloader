/* Me re-writing everything with GNU Assembler because I hate my life */
.intel_syntax noprefix 
.org 0x0 
.section .text
.global _start


.macro SANITY_CHECK /* Sanit Check */
    mov ah, 0x0e
    mov al, '\n'
    int 0x10
    mov al, 'O'
    int 0x10
    mov al, 'K'
    int 0x10
    mov al, '\n'
    int 0x10
.endm

/* Not sure why this isn't working, it's like the bios isn't even going to start which is loaded at 0x7c00?? */
_start:
    jmp _main /* For some reason ljmp makes the bootloader bug out */

_main:
/* Make sure that the segment registers are 0 because we don't know what the bios has done with them before handing control over to the bootloader */
    xor ax, ax
    mov ss, ax 
    mov sp, 0x7c00
    mov es, ax
    mov ds, ax

    SANITY_CHECK

    jmp . 

.org 510 /* GNU Assembler is retarded, this fills the binary with bytes until the local address is 510 */
.word 0xaa55 /* We will pass little endian as an argument to the compiler that way this is formated properly for the BIOS */
