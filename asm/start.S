.global init_cp15
.global decompress_code
.global init_data_sections
.global DAT_02004ad8

.global _start

_start:
    // Disable interrupts by clearing bit 0 of the IME register
    mov        r12, #0x04000000
    str        r12, [r12, #0x208]
    
    bl         init_cp15

    // Initialize stack for supervisor mode
    mov        r0, #0x13
    msr        cpsr_c, r0
    ldr        r0, DAT_020048b8
    add        r0, r0, #0x3fc0
    mov        sp, r0
    
    // Initialize stack for IRQ mode
    mov        r0, #0x12
    msr        cpsr_c, r0
    ldr        r0, DAT_020048b8
    add        r0, r0, #0x3fc0
    sub        r0, r0, #0x40
    mov        sp, r0

    // Initialize stack for system mode
    ldr        r1, DAT_020048bc
    sub        r1, r0, r1
    mov        r0, #0x1f
    msr        cpsr_cxsf, r0
    sub        sp, r1, #4

    // Decompress the main code
    ldr        r1, PTR_DAT_020048c0
    ldr        r0, [r1, #0x14]
    bl         decompress_code

    // Initialize .data and .bss
    bl         init_data_sections

    mov        r0, #0
    ldr        r1, DAT_020048b8
    mov        r2, #0x4000
    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x53c80
    
    mov        r0, #0
    ldr        r1, DAT_020048c4
    mov        r2, #0x400
    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x53c70
    
    mov        r0, #0x200
    ldr        r1, DAT_020048c8
    mov        r2, #0x400
    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x53c60

    // Push some value to the IRQ stack? Doesn't update the sp though
    ldr        r1, DAT_020048b8
    add        r1, r1, #0x3fc0
    add        r1, r1, #0x3c
    ldr        r0, DAT_020048cc
    str        r0, [r1]

    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x3ff672c
    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x14d0c
    // TODO: this branch offset is hardcoded because it branches to a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    bl         #0x6c63c

    // Call main, reset on return
    ldr        r1, PTR_FUN_020048d0
    ldr        lr, PTR_H_Reset_020048d4
    bx         r1

DAT_020048b8:
    .word 0x023c0000
DAT_020048bc:
    .word 0x00000600
PTR_DAT_020048c0:
    .word DAT_02004ad8
DAT_020048c4:
    .word 0x05000000
DAT_020048c8:
    .word 0x07000000
DAT_020048cc:
    .word 0x01ffd8e0
PTR_FUN_020048d0:
    // TODO: this function pointer is hardcoded because it is a function inside the
    // decompressed code. Should be possible to determine the offset at compile time.
    .word 0x02007000
PTR_H_Reset_020048d4:
    .word 0xffff0000
