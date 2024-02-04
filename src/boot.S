.code16
.global  _start
.section .text

_start:
/*
This sets the code segment to 0 that way we know the value of it 
*/
	ljmp $0, $_fix_cs_register

_fix_cs_register:
/*
Clear the direction flag so we are moving forward in memory when dealing with strings 
*/
	cld
/*
Zero ax and then move the value in ax into all of the segment registers that way we know their value 
*/
	xor %ax, %ax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %ss
/*
Move the stack pointer to point at the start of our bootloader, from here the stack grows downwards underneath our bootloader 
*/
	mov $0x7c00, %sp
/*
Set the base pointer to the stack pointer 
*/
	mov %sp, %bp
/*
We compare dl to see if we are booting from a hard drive: dl >= 0x80 
*/
	cmp $0x80, %dl
/*
If not, spin the CPU 
*/
	jb __spin
/*
Any value above 0x8f are dubious
*/
	cmp $0x8f, %dl
/*
Spin if the value is greater than 0x8f
*/
	ja __spin
/*
Move the disk drive value to somewhere we can grab it later in case dl is trashed 
*/
	movb %dl, _numer_of_boot_drive
/*
Check if 13h extensions are supported 
*/
	call _check_int13h_extension
/*
Read the 32 bit mode from disk 
*/
	call _read_from_disk
/*
	 Turn interupts off to enable 32 bit mode  
 */
	cli
/*
	 Load the global descriptor table 
 */
	lgdtw __load_gdt
/*
Enable 32 bit mode 
*/
	mov %cr0, %eax
	or  $1, %eax
	mov %eax, %cr0
/*
	 Long jump to the 32 bit section of the program, this is loaded directly after our bootloader in memory (0x7c00 + 512d == 0x7e00) 
 */
	ljmp $0x08, $_32bit_start

/*
__spin is needed and called by a lot of the labels
Also fall through if the jump to 32 bit mode fails? 
*/
__spin:

	mov  _spin_msg, %si
	call _print
	jmp  .

_spin_msg:
	.asciz "spin\n\r"
/*
STOP FORMATING MY SHIT NEOVIM FUCK OFF
*/
	.include "./src/gdt.inc"
	.include "./src/print.inc"
	.include "./src/disk.inc"

	.code32
	.global _32bit_start

_32bit_start:
	movb $'g', (0xb8000)
	movb $0x0f, (0xb8001)
	movb $'g', (0xb8002)
	movb $0x0f, (0xb8003)

__32bit_loop:
	hlt
	jmp __32bit_loop

.org  510
.word 0xaa55

.org 1000
