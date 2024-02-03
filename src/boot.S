.code16
.global  _start
.section .text

/*
	 TODO: 
	 Unfortunetly it seems that a program I have that formats my code corrects the multi-line comments to single line comments, however single line comments arent supported by GCC so I will need to re-write my comments 
	 Until then please refer to old comments in old commits for information 

 */
.macro SANITY_CHECK
mov    $0x0e, %ah
mov    $'[', %al
int    $0x10
mov    $'O', %al
int    $0x10
mov    $'k', %al
int    $0x10
mov    $']', %al
int    $0x10
mov    $'\n', %al
int    $0x10
mov    $'\r', %al
int    $0x10
.endm

_start:

	SANITY_CHECK

	ljmp $0, $_main

	SANITY_CHECK

_main:

	SANITY_CHECK

	cld

	xor %ax, %ax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %ss

	mov $0x7c00, %sp

	mov %sp, %bp

	cmp $0x80, %dl

	jb __spin

	cmp $0x8f, %dl

	ja __spin

	movb %dl, _numer_of_boot_drive

	call _check_int13h_extension

	call _read_stage2_from_disk

	mov  $_bootloader_hello_world, %si
	call _print

/*
	 Turn instructions off 
 */
	cli
/*
	 Load the global descriptor table 
 */
	lgdt __gdt_end

/*
	 I have no idea if this is going to work, so I guess pray :) 
 */
	ljmp $0, $_32bit_start

__spin:
	jmp .

_print:

	mov $0x0e, %ah

_print_loop:
	lodsb
	cmp $0, %al
	je  _print_done
	int $0x10
	jmp _print_loop

_print_done:
	ret

_check_int13h_extension:
	mov $0x41, %ah
	mov $0x55aa, %bx
	mov $0x80, %dl
	int $0x13

	jc _check_int13h_extension_error

_check_int13h_extension_done:
	ret

_check_int13h_extension_error:
	mov  $_int13h_err_msg, %si
	call _print
	jmp  __spin

_read_stage2_from_disk:
	mov $0x02, %ah
	mov _numer_of_boot_drive, %dl
	mov $0x00, %ch
	mov $0x00, %dh
	mov $0x02, %cl
	mov $0x01, %al
	xor %bx, %bx
	mov %bx, %es
	mov $0x7e00, %bx
	clc
	int $0x13
	jc  _read_from_disk_error
	cmp $0x01, %al
	jne _read_from_disk_error2

	mov  $_bootloader_disk_ok, %si
	call _print
	ret

_read_from_disk_error:
	mov  $_bootloader_disk_error, %si
	call _print
	ret

_read_from_disk_error2:
	mov  $_bootloader_disk_error2, %si
	call _print
	ret

_bootloader_hello_world:
	.asciz "Bootloader Hello World!\n\r"

_int13h_err_msg:
	.asciz "int 0x13 extensions are not supported!\n\r"

_bootloader_disk_ok:
	.asciz "[~] loaded disk without error or something :)\n\r"

_bootloader_disk_error:
	.asciz "[!] A disk error occured! (1)\n\r"

_bootloader_disk_error2:
	.asciz "[!] A disk error occured! (2)\n\r"

	_numer_of_boot_drive: .byte 0

/*
	 Please read this internet archive: http://web.archive.org/web/20190424213806/http://www.osdever.net/tutorials/view/the-world-of-protected-mode
 */
__gdt_start:
/*
	 The first 8 bytes of the GDT should be initialized to NULL. Each word should be 2 bytes. 
 */
	.word 0x0
	.word 0x0
	.word 0x0
	.word 0x0

/*
	 The first dword specifies the limit of the code segment 
	 This states that we want our code segment to start at 0x0 and to go to 0xffff (4gb)
 */
	.word 0xff
	.word 0xff
/*
	 The second dword specifies the base of the segment, in this case we are starting at 0x0 (the start of memory)
 */
	.word 0x0
	.word 0x0
/*
	 This byte continues on the base address of the segment. Note: this entire section is one dword
 */
	.byte 0x0
/*
	 0x9a is the hex representation of the byte: 10011010
	 TODO: explaing what each of the bits represent 
 */
	.byte 0x9a
/*
	 0xcf is the hex representation of the byte: 11001111
 */
	.byte 0xcf
/*
	 I have no idea what this byte is for, however it is the last byte in the dword 
 */
	.byte 0x0
/*
	 These bytes specify the data segment information
	 The first two dwords are the same as the code segment
 */
	.word 0xff
	.word 0xff
	.word 0x0
	.word 0x0
/*
	 Same for the first 8 bits of the second double word 
 */
	.byte 0x0
/*
	 This is the hex representation of: 10010010
 */
	.byte 0x92
/*
	 This is the hex representation of: 11001111
 */
	.byte 0xcf
/*
	 This byte ends the second dword
 */
	.byte 0x0

__gdt_end:
/*
	 Current address - GDT start - 1 because length starts at 1 
 */
	.word . - __gdt_start - 1
	.int  __gdt_start

	.code32

_32bit_start:
	mov $'f', (0x8b000)

__32bit_loop:
	jmp __32bit_loop
hlt

_hello_from_stage2:
	.asciz "Hello from stage2"

	.org 510

	.word 0xaa55

	.org 510 + 200
