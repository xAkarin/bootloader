/* Me re-writing everything with GNU Assembler because I hate my life */
.intel_syntax noprefix 
.org 0x0 
.section .text
.global _start

_start:
    mov al, 'c'
    int 0x10
    jmp . 

.org 510 /* GNU Assembler is retarded, this fills the binary with bytes until the local address is 510 */
.byte 0x55 /* Magic bytes in **little endian** */
.byte 0xaa
