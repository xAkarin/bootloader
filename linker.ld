ENTRY(__asm_entry)

SECTIONS { 
  . = 0x7c00; 
  __asm_entry = .; 
  .asm : {
      *(.asm*)
  }
}
