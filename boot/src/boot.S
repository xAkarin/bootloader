/* Me re-writing everything with GNU Assembler because I hate my life */
.intel_syntax noprefix 
.org 0x0 
.section .text
.global _start

/* I am losing my goddamn mind */ 

.macro SANITY_CHECK /* Sanity Check */
    mov ah, 0x0e
    mov al, 'O'
    int 0x10
    mov al, 'K'
    int 0x10
    mov al, '\n'
    int 0x10
    mov al, '\r'
    int 0x10
.endm

_start:
    jmp _main /* For some reason ljmp makes the bootloader bug out */

    /* For now I cannot long jump to set the code segment without fully understanding why it bugs out, 
    *  so I just wont and I'll leave it until I absolutely must 
    *
    */


 /* the bootloader SHOULD NOT go over this macro 
 *  if it does then I know that there is a SEVRE problem with how my program is jumping
 */
    SANITY_CHECK

_main:
/* Make sure that the segment registers are 0 because we don't know what the bios has done with them before handing control over to the bootloader */
    
    cld /* Change the direction bit to 0 (go forward in memory when dealing with strings)*/

    xor ax, ax
    mov ss, ax 
    mov sp, 0x7c00
    mov es, ax /* Not clearing this segment breaks the fucking bootloader GOD WHY IS GAS SO FUCKING ASSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS THIS DOESNT HAPPEN IN NASM FUCKING MORASDAASDASDa */
    mov ds, ax
    
    /* Ensuring that it actually jumped to the main */
    SANITY_CHECK 

    mov si, _boot_msg
    /* TODO: for some retarded reason "call" just fucking dies and doesn't work */
    call _check_call

    jmp .

/* Bootloader print function */
_print:
SANITY_CHECK /* It is not making it here :( */
/*
    lodsb 
    cmp al, 0
    je _print_done 
    int 0x10 
    jmp _print 

    The above assembler does not work, why? I have no ide. GNU Assembler was made by idiots
    and appears to continue to set the standard for RETARDED and USELESS open source 
    software. 
*/

/* Ensure we are in teletype mode */

mov ah, 0x0e 

_print_loop:
/*SANITY_CHECK*/
    mov ax, si
    cmp al, 0
    je _print_done 
    inc si
    int 0x10 
    jmp _print_loop

_print_done: 
   ret

/* A function for debugging to see if call is working :) god I hate the people who made GAS */
_check_call: 
    SANITY_CHECK
    ret 

_boot_msg: .asciz "Boot Message\n"

.org 510 /* GNU Assembler is retarded, this fills the binary with bytes until the local address is 510 */
.word 0xaa55 /* We will pass little endian as an argument to the compiler that way this is formated properly for the BIOS */
