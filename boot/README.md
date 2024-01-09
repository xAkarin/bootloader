## Reasources: 

1. https://stackoverflow.com/questions/33603842/how-to-make-the-kernel-for-my-bootloader
2. https://wiki.osdev.org/Rolling_Your_Own_Bootloader
3. https://stackoverflow.com/questions/57237499/how-to-properly-setup-ss-bp-and-sp-in-x86-real-mode
4. https://thestarman.pcministry.com/asm/debug/Segments.html
5. https://www.cs.cmu.edu/~410-s07/p4/p4-boot.pdf
6. https://www.cs.princeton.edu/courses/archive/fall09/cos318/precepts/precept0
7. https://wiki.osdev.org/Real_Mode#Switching_from_Protected_Mode_to_Real_Mode
8. https://github.com/cfenollosa/os-tutorial/issues/269
9. https://stackoverflow.com/questions/1395591/what-exactly-is-the-base-pointer-and-stack-pointer-to-what-do-they-point
10. https://stackoverflow.com/questions/56008686/printing-a-string-in-16-bit-real-mode
11. https://stackoverflow.com/questions/39281414/why-does-my-bootloader-crash-after-adding-this-line\
12. https://stackoverflow.com/questions/14060411/x86-where-stack-pointer-points
13. https://stackoverflow.com/questions/7279311/difference-between-bx-and-bp
14. https://en.wikibooks.org/wiki/X86_Assembly/16,_32,_and_64_Bits

# Verifying that the stack grows downwards: 
```asm
;
; assume that ss = 0 and that sp = 0x7c00 (so that the stack grows bellow our bootloader)
;
verify:
    push 'a'
    mov al, [0x7bfe] ; verifying the stack grows downwards
    cmp al, 'a'
    je .intd
    jmp .end
.intd:
    int 0x10 ; should fall through into end
.end:
    pop ax   ; pop value off the stack into ax and zero it
    xor ax, ax
```





