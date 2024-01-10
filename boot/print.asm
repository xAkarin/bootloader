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
    ret