.section .start_block, "a", %progbits
.global __start_block
__start_block:
    .word 0xffffded3 // PICOBIN_BLOCK_MARKER_START
    .byte 0x42
    .byte 0x1
    .hword 0b0001000000100001
    .byte 0xff
    .hword 0x0001
    .byte 0
    .word 0
    .word 0xab123579 // PICOBIN_BLOCK_MARKER_END
