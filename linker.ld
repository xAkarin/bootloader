ENTRY(_start)

SECTIONS {
  . = 0x7c00; 
  .text : { *(.text*) } /* This translates to "take all of the text segments in every file and paste it here"
                         * for obvious reasons, when I have a second stage bootloader I need to spend the time and read the LD manual so that my shit doesnt break agian
                         */ 
}
