bits 16
org 0x7c00

jmp 0:start ; should set the CS segment properly? 

start: 
    xor ax, ax
    mov ss, ax
    mov sp, 0x7c00
    mov ds, ax
    mov bp, sp

    mov si, msg
    call print 

print:
    mov ah, 0x0e 
.loop: 
    cmp byte [si], 0 ; this is legal??? 
    je .end
    mov al, [si]
    int 0x10
    inc si 
    jmp .loop  
.end: 

%define ENDL 0x0d, 0x0a
msg: 
    db "[!] MESSAGE", ENDL, 0 

times 510 - ($ - $$) db 0
dw 0xaa55   