 bits 16
 org 0x7c00 
 bootmain:
    mov bp, 0x9000
    mov sp, bp
    call bcls 
    mov si, example
    call bprint
    jmp halt 

halt: 
    jmp halt

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
times 510 - ($-$$) db 0
dw 0xaa55