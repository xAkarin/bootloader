bits 16

org 0x7c00

xor ax, ax
mov cs, ax
mov ss, ax
mov sp, ax
mov ds, ax

push 'a'

mov ax, [sp]

int 0x10

times 510 - ($ - $$) db 0
dw 0xaa55