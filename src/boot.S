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
    /* Set the stack pointer to our bootloader address (the stack grows downward from here and should be growing down by 16 bits) */ 
    mov $0x7c00, %sp
    /* Set the stack base pointer to the stack pointer (both should originally point at whatever value is stored in sp) */ 
    mov %sp, %bp

    /* Instead we should be checking to see if DL is a hardrive I guess lol */
    cmp $0x80, %dl /* This would be the lowest number DL could be if it was a harddrive we where booting off of */

    /* TODO: REMEMBER THAT IT IS DL NOOOOOOOOOOOOOOOOT Di BECAUSE OMG I AM GOING TO HATE MY LIFE IF I MESS THAT UP AGAIN */
    
    /* We will know if it wasn't booting off of a hard drive if it doesnt print the bios hello world*/
    /* Jump if bellow (if the value in dl is smaller) */
    jb __spin

    /* The limine bootloader states that values above 0x8f are probably just wack and shouldn't be trusted, so also error on that I guess */
    cmp $0x8f, %dl 

    /* Jump if above (if the value in dl is larger) */
    ja __spin 

    /* So actually this is useless I think */
    /* TODO: remember why I have to do this? */
    /* Move the boot drive number into a variable to ensure I don't lose it */
    /* This should default to 0x0 based off of what I've read, however I don't beleive that to be the case */
    movb %dl, _numer_of_boot_drive

    /* Call this to make sure int13h extensions are enabled */
    call _check_int13h_extension
    /* Calling this before %dl is trashed */
    call _read_stage2_from_disk

    mov $_bootloader_hello_world, %si 
    call _print

    /* LETS FUCKING GOOOOOOOOOOOOOO IT WORKS HAHAHAHAHAHAH LETS GOOOOOOOOOOOOOOOOOOOOOOOOOO */
    /* Just to see if it did load */
    ljmp $0, $0x7e00



/* fall through into spin, spin is also used for errors and shiiiiit */
__spin:
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

/* a label to check if the CPU supports 13h extensions for disk addressing? 
SUPER USEFUL https://handwiki.org/wiki/INT_13H#List_of_INT_13h_services
*/ 
_check_int13h_extension:
  /* Move this to test whether extensions are available */
  mov $0x41, %ah
  /* Another magic number for checking if the extensions are supported I think */
  mov $0x55aa, %bx
  /* This is the DEFAULT HARD DRIVE, NOT FLOPPY DISK */
  mov $0x80, %dl 
  /* Call the interupt to call the bios function */
  int $0x13 

  /* the carry flag is set if int 0x13 extensions are not available */
  jc _check_int13h_extension_error

/* Else just fall through into this label and return */
_check_int13h_extension_done:
  ret

_check_int13h_extension_error:
  mov $_int13h_err_msg, %si 
  call _print 
  jmp __spin /* if 13h extensions are not supported, then just spin because yeah idk fuck u */

/* This is meant to be my read from disk function but currently it just throws and error or something idk */
/* So like it appears to work, but I actually have 0 clue why it's throwing an error, maybe I need to make a TODO: "print_hex" function to print the value? */
_read_stage2_from_disk: 
    /* Bios magic number for the read sectors from drive function */
    mov $0x02, %ah 
    /* Move the drive we booted off of into %dl to be read. Our hard disk drive number??? */
    /* OKAY, so it has to be the harddrive number that's GREAT news, now it's only erroring for number 2 */
    /* IMPORTANT: make sure that DL is the HARDDRIVE number, TODO: we can't be 100% sure that the harddrive number is infact 0x80, but that's for later*/
    /* Moving the saved value back into DL in case it was trashed */
    mov _numer_of_boot_drive, %dl
    /* ch is the register that you place the cylinder number in */
    /* Read cylinder 0 (1 as it starts at 0) as this is the cylinder that our stage 2 will be on */
    mov $0x00, %ch 
    /* Select the track on the top side of a floppy disk. This starts at 0. DH is the register you place the head in */ 
    /* I think that our stage2 is on the top side of the floppy, but floppy addressing is hard to understand */
    mov $0x00, %dh 
    /* Select the second sector of the track. This starts at 1. CL is the register you the start sector in */
    /* The reason we are reading the second sector of the track is because the first sector SHOULD be the one holding our bootloader stage1 */
    mov $0x02, %cl 
    /* This is the number of sectors we read from: Cylinder 3, Head 1, Sector 4 */
    /* In otherwords, we should read 5 sectors from the above stated location */
    /* This reads 1 sector off of the disk */
    /* The reason it is currently 1 sector is because my bootloader is not big enought to be more than 2 sectors large :) */
    mov $0x01, %al 
    /* Set the address that the bios reads the sectors to. 
    I assume this works like how a binary is filled out, 
    as in from memory address start -> end is litterally just pasted from the disk. 
    */
    /* Set the es segment register to 0 by 0ing BS and then moving 0 into es*/
    xor %bx, %bx 
    /* Here we 0 es indirectly*/
    mov %bx, %es 
    /* Then we load where we want our second stage bootloader to go  into memory */
    /* This is directly above the first stage bootloader in memory :) */
    mov $0x7e00, %bx 
    /* Clear the carry flag just to MAKE sure that it is throwing an error */
    clc
    /* Call the interupt to read from disk and load into the correct memory address, pray this works I guess :) */
    int $0x13
    /* The carry flag is set if there was an error reading from disk */
    jc _read_from_disk_error
    /* Here we are comparing the total number of sectors we wanted to read off of the disk to what was actually read */
    cmp $0x01, %al
    /* If the total number of sectors does not equal the amount of sectors we wanted to read off of the disk then an error occured and we jump to an error message */
    jne _read_from_disk_error2

/* otherwise we fall through to read from disk OK and print and OK message to the screen to let me know (for my sanity) that an error did not occur */
/* If an error did occur you can find the error number in AX and then lookup what that value means, I havent implemented a way to print hex yet so Im just gonna wing it :)*/
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

_int13h_err_msg:
  .asciz "int 0x13 extensions are not supported!\n\r"

_bootloader_disk_ok: 
  .asciz "[~] loaded disk without error or something :)\n\r"

/* Bootloader disk error message for sanity checking */
_bootloader_disk_error:
  .asciz "[!] A disk error occured! (1)\n\r"

/* Bootloader disk error 2 for sanity checking named differently with a different value to distinguish differences in errors */
_bootloader_disk_error2:
  .asciz "[!] A disk error occured! (2)\n\r"

_numer_of_boot_drive: .byte 0


/* Pad the binary with bytes until it is 510 bytes in size */
.org 510 
/* Then add the magical MBR word value to the end of the binary making it 512 bytes and bootable */ 
.word 0xaa55

/* the bios will only load the first 512 bytes, so I'm going to place a few bytes after the bootloader to jmp to in order to check that something was actually loaded to the correct
   memory address from the disk 

   Stuff after this point would be past the first 512 bytes, and as such would be read off of the disk and then loaded to 0x7e00 
*/

/* 
mov $_hello_from_stage2, %si 
call _print 
*/ 
__stage2_loop: 
  jmp __stage2_loop /* Jump to current address / spin the CPU */
  hlt 
/*
_hello_from_stage2:
  .asciz "Hello from stage2"
*/
