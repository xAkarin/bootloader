 bits 16

 org 0x07c00 

 bootmain:
    mov bp, 0x9000
    mov sp, bp

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
    jmp halt 

example: db "Hello World", 0

times 510 - ($-$$) db 0

dw 0xaa55