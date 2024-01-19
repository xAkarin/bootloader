.code16
.global _start 
.section .text 

.macro SANITY_CHECK
    mov $0x0e, %ah 
    mov $'[', %al 
    int $0x10 
    mov $'O', %al 
    int $0x10 
    mov $'k', %al 
    int $0x10
    mov $']', %al 
    int $0x10
    mov $'\n', %al 
    int $0x10 
    mov $'\r', %al 
    int $0x10 
.endm 

_start:
    SANITY_CHECK

    ljmp $0, $_main

/* HOLY SHIT ITS WORKING AHAHAUISGBDAJHSDJHADF  FSIUNABKL:YJNYUK FIANLLY JOAHSIUDGABY SIUJHBAHBSDD IDK WHAT I DID BUT ITS FUCKING WORKING LETS GOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO! */
_main:

    SANITY_CHECK

    jmp .

.org 510 
.word 0xaa55 
