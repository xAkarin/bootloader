ENTRY(__asm_entry)

SECTIONS {
    . = 0x7c00;

    .asm :
    {
        *(.asm .asm.*)
    }
    .rust :
    {
        *(.text .text.*)
        *(.rodata .rodata.*)
    }

    /DISCARD/ : {
        *(.comment .comment.*)    
    }

    . = 0x7c00 + 510-16;

    _partition_table = .;
    .partition_table :
    {
        /* partition table entry 4 */
        QUAD(0)
        QUAD(0)
    }

    .magic_word : {
        SHORT(0xaa55)
    }
    
}
