/* Me re-writing everything with GNU Assembler because I hate my life */
.intel_syntax noprefix 
.org 0x0 
.section .text
.global _start

/* Not sure why this isn't working, it's like the bios isn't even going to start which is loaded at 0x7c00?? */
_start:
    mov al, 'c'
    int 0x10
    jmp . 

.org 510 /* GNU Assembler is retarded, this fills the binary with bytes until the local address is 510 */
.word 0xaa55 /* We will pass little endian as an argument to the compiler that way this is formated properly for the BIOS */
