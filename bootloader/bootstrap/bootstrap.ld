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

    . = 0x7c00 + 510;

    .magic_word : {
        SHORT(0xaa55)
    }
    
}
