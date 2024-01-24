.code16
.global _start 
.section .text 

/* This is a form of debugging macro, just used to ensure that the program flow is correct */
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
    /* Ensure that the bootloader is actually running from the correct address */ 
    SANITY_CHECK

    /* This jump will 0 the code segment that way when calling a label the address will be (0 * 16):(+ offset) */
    ljmp $0, $_main

    /* The code jumps over this, this ensures that the ljmp actually occured and we did not fall into _main */
    SANITY_CHECK
    
_main:
    /* Ensure that it did infact jump to main */ 
    SANITY_CHECK
    
    /* Clear the direction bit (set it to 0). This ensures we are moving forward in memory when dealing with strings and other things */
    cld 

    /* Here we zero out the segment registers as the values given to us by the bios are unknown */
    xor %ax, %ax 
    mov %ax, %ds 
    mov %ax, %es 
    mov %ax, %ss

    /* TODO: remember why I have to do this? */
    /* Move the boot drive number into a variable to ensure I don't lose it */
    /* This should default to 0x0 based off of what I've read, however I don't beleive that to be the case */
    mov %di, (_numer_of_boot_drive)

    /* Set the stack pointer to our bootloader address (the stack grows downward from here and should be growing down by 16 bits) */ 
    mov $0x7c00, %sp
    /* Set the stack base pointer to the stack pointer (both should originally point at whatever value is stored in sp) */ 
    mov %sp, %bp

    mov $_bootloader_hello_world, %si 
    call _print

    call _read_from_disk 

    jmp .

/* Bootloader print message */
_print: 
    /* ensure we are in teletype mode */ 
    mov $0x0e, %ah 
_print_loop: 
    /* Load byte at memory address in %si, into %al then increment %si by 1 byte */
    lodsb 
    /* Compare %al to $0 (null) */
    cmp $0, %al 
    /* If (%al == NULL){ jmp _print_done } */
    je _print_done
    /* else { use 0x10 interupt to print value in al to screen }*/ 
    int $0x10 
    /* Loop */
    jmp _print_loop
_print_done:
    /* Return to the address that was pushed onto the stack by the call instruction */
    ret 

/* TODO: note to self 
   mov ah, 0x02    ; BIOS read sector function
   mov dl, 0        ; Read drive 0 ( i.e. first floppy drive )
   mov ch, 3        ; Select cylinder 3
   mov dh, 1        ; Select the track on 2 nd side of floppy
                    ; disk, since this count has a base of 0
   mov cl, 4        ; Select the 4 th sector on the track - not
                    ; the 5 th , since this has a base of 1 .
   mov al,5         ; Read 5 sectors from the start point
                    ; Lastly , set the address that we ’d like BIOS to read the
                    ; sectors to , which BIOS expects to find in ES : BX
                    ; ( i.e. segment ES with offset BX ) .
   mov bx, 0xa000   ; Indirectly set ES to 0 xa000
   mov es, bx
   mov bx, 0x1234   ; Set BX to 0 x1234
                    ; In our case , data will be read to 0 xa000 :0 x1234 , which the
                    ; CPU will translate to physical address 0xa1234 int 0x13 
                    ; Now issue the BIOS interrupt to do the actual read.
  
  I wont know if this works until I load stage 2 into memory and call it. 

  Also TODO: what the fuck does this mean? 

*/

/* This is meant to be my read from disk function but currently it just throws and error or something idk */
_read_from_disk: 
    /* Bios read sector function */
    mov $0x02, %ah 
    /* Move the drive we booted off of into %dl to be read */
    mov (_numer_of_boot_drive), %dl 
    /* Read cylinder 3 */
    mov $0x03, %ch 
    /* Select the track on the second (bottom side) of a floppy disk. This starts at 0 */ 
    mov $0x01, %dh 
    /* Select the fourth sector of the track. This starts at 1 */
    mov $0x04, %cl 
    /* Read 5 sectors from the start ?? */
    mov $0x05, %al 
    
    /* Set the address that the bios reads the sectors to. 
    I assume this works like how a binary is filled out, 
    as in from memory address start -> end is litterally just pasted from the disk. 
    */
    /* Currently some random ass address */
    mov $0x0a000, %bx 
    mov %bx, %es 
    mov $0x1234, %bx 
    /* The bios expects to find the address to move the stuff read from disk in es:bx 
       so here we are just setting es:bx to some random ass values to test if it is even working I guess 
    */

    /* Call the interupt to read from disk and load into the correct memory address, pray this works I guess :) */
    int $0x13

    jc _read_from_disk_error

    cmp %dh, %al
    jne _read_from_disk_error2

_read_from_disk_done:
  mov $_bootloader_disk_ok, %si 
  call _print 
  ret 

_read_from_disk_error:
  mov $_bootloader_disk_error, %si 
  call _print 
  ret 

 _read_from_disk_error2:
  mov $_bootloader_disk_error2, %si 
  call _print 
  ret 

/* Bootloader hello world message to test if the print function is working */
_bootloader_hello_world: 
    .asciz "Bootloader Hello World!\n\r"

_bootloader_disk_ok: 
  .asciz "[~] loaded disk without error or something :)\n\r"

/* Bootloader disk error message for sanity checking */
_bootloader_disk_error:
  .asciz "[!] A disk error occured! (1)\n\r"

/* Bootloader disk error 2 for sanity checking named differently with a different value to distinguish differences in errors */
_bootloader_disk_error2:
  .asciz "[!] A disk error occured! (2)\n\r"

_numer_of_boot_drive: .word 0 

/* Pad the binary with bytes until it is 510 bytes in size */
.org 510 
/* Then add the magical MBR word value to the end of the binary making it 512 bytes and bootable */ 
.word 0xaa55

/* The bios will only load the first 512 bytes, so as long as we have the magic word and ensure that the beggining of our binary is within those first 512 bytes it doesnt matter how large the binary is 
   Also this has got to be stupid, I'm litterally making this several killobytes large lmfao

   Sp even making our binary large as fuck doesnt really do anything??? Maybe I should make it a few GB is size ... like an idiot ...

   LMAO ITS 4 GIGS IN SIZE NOW AND TOOK LIKE 10 SECONDS TO ASSEMBLE FUUUUUUUUUUUUUUUUUUUUUUUUUUUUCK
*/
.org 0xffffffff 
