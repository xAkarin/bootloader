/*
Everything pertaining to reading the disk in 16 bit real mode assembly
This file depends on print.S as it uses the print labe!
*/
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

_read_from_disk:
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

_int13h_err_msg:
	.asciz "int 0x13 extensions are not supported!\n\r"

_bootloader_disk_ok:
	.asciz "[~] loaded disk without error or something :)\n\r"

_bootloader_disk_error:
	.asciz "[!] A disk error occured! (1)\n\r"

_bootloader_disk_error2:
	.asciz "[!] A disk error occured! (2)\n\r"

	_numer_of_boot_drive: .byte 0
