 bits 16
 ; boot drive stored from dl
 mov [BOOT_DRIVE], dl 
 ; org where its loaded or smth aaaaaaaa
 org 0x7c00 

 ; setup stack I guess or something fucking stupid annoying fuck you
mov bp, 0x9000
mov sp, bp

bootmain:
    ; call boot clear screen
    call bcls 
    ; call boot print *example text*
    mov si, example
    call bprint
    
; keyboard input loop
.kbi_loop: 
    mov ah, 0x00
    int 0x16

    mov ah, 0x0e
    int 0x10

    jmp .kbi_loop

bprint: 
    cld  
    mov ah, 0x0e
    lodsb 
    or al, al
    jz .ret
    int 0x10
    jmp bprint
.ret:
    ret 

bcls: 
    mov ah, 0x00
    mov al, 0x03
    int 0x10
.ret: 
    ret

example: db "example", 0
BOOT_DRIVE: db 0 
times 510 - ($-$$) db 0
dw 0xaa55