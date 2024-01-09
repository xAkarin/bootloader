bits 16
org 0x7c00

jmp 0:start ; should set the CS segment properly? 

start: 
    xor ax, ax
    mov ss, ax
    mov sp, 0x7c00
    mov ds, ax
    mov bp, sp

    mov ah, 0x0e

verify:
    push 'a'
    mov al, [0x7bfe] ; verifying the stack grows downwards
    cmp al, 'a'
    je .int
    jmp .end
.int:
    int 0x10 ; should fall through into end
.end:
    pop ax   ; pop value off the stack into ax and zero it
    xor ax, ax 

times 510 - ($ - $$) db 0
dw 0xaa55   