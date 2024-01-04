bits 16

org 0x7c00

xor ax, ax
mov cs, ax
mov ss, ax
mov sp, 0x7c00
mov ds, ax

mov bp, sp

push 'a'

mov ax, [ss:bp]
 
int 0x10

times 510 - ($ - $$) db 0
dw 0xaa55   