bits 16
org 0x7c00

jmp 0:start ; should set the CS segment properly? 

start: 
    xor ax, ax
    mov ss, ax
    mov sp, 0x7c00
    mov ds, ax
    mov bp, sp

    cld ; ensure that incrementing an address increases it (direction bit = 0) 

    mov si, msg
    call print 

;.hlt:
;    jmp .hlt

%include "print.asm" 

%define ENDL 0x0d, 0x0a
msg: 
    db "[!] MESSAGE", ENDL, 0 

times 510 - ($ - $$) db 0
dw 0xaa55   