.align 8
__load_gdt:
	.word (__gdt_end - __gdt_start) - 1
	.long  __gdt_start

/*
	 Please read this internet archive: http://web.archive.org/web/20190424213806/http://www.osdever.net/tutorials/view/the-world-of-protected-mode
   Please read this wikipedia article: https://en.wikipedia.org/wiki/Global_Descriptor_Table
   Please read this OSDev article: https://wiki.osdev.org/GDT_Tutorial
 */
__gdt_start:

/*
The first 8 bytes of the GDT should be full of 0's 
*/ 
__gdt_null: 
	.quad 0x0000000000000000

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

