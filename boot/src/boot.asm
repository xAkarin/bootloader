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

    ; enables a20? 
    in al, 0x92
    or al, 2
    out 0x92, al

    mov si, msg
    call print 

.hlt:
    jmp .hlt

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

check_a20:
    cli ; ? Not sure dont want to edit the registers in any way? 

    ; make it so that es:si == 0x0000:0x0500
    xor ax, ax ; 0
    mov es, ax
    mov si, 0x0500

    ; make it so that ds:di == 0xffff:0x0510 which is == 0x0000:0x0500
    mov ax, 0xffff
    mov ds, ax
    mov di, 0x0510

    ; here we are just moving the memory at these locations onto the stack to be restored later
    ; why is the thing Im looking at saying it needs to be a pointer? 
    mov al, byte [es:si]
    push ax

    mov al, byte [ds:di]
    push ax

    ; this is where we test if memory wraps around
    mov [es:si], byte 0x00 
    mov [ds:di], byte 0xff

    ; if memory wraps around, es:si will now be 0xff, if it doesn't wrap around
    ; then es:si will be 0x00 and the location ds:di points at will be 0xff
    cmp [es:si], byte 0xff 

    pop ax
    mov [ds:di], al  

    pop ax
    mov [es:si], al

    mov ax, 0

    je .exit

    mov ax, 1
.exit: 
    ret 







%define ENDL 0x0d, 0x0a
msg: 
    db "[!] MESSAGE", ENDL, 0 

times 510 - ($ - $$) db 0
dw 0xaa55   