/*
	 Please read this internet archive: http://web.archive.org/web/20190424213806/http://www.osdever.net/tutorials/view/the-world-of-protected-mode
 */
__gdt_start:

__gdt_null:
	.word 0x0000
	.word 0x0000
	.word 0x0000
	.word 0x0000

__gdt_code:
	.word 0xffff
	.word 0x0000
	.byte 0x00
	.byte 0x9a
	.byte 0xcf
	.byte 0x00

__gdt_data:
	.word 0xffff
	.word 0x0000
	.byte 0x00
	.byte 0x92
	.byte 0xcf
	.byte 0x00

__gdt_end:
/*
	 Current address - GDT start - 1 because length starts at 1 
 */
	.word (__gdt_end - __gdt_start) + 1
	.int  __gdt_start
