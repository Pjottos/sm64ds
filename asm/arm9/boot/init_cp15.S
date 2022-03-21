.global _COMPRESSED_END_

.global init_cp15
.global DAT_02004ad8

init_cp15:
    // Clear the following bits in the cp15 control register:
    // 0 - disable protection unit
    // 2 - disable data cache
    // 12 - disable instruction cache
    // 15 - set the pre-armv5 mode to normal
    // 16 - disable DTCM (data tightly coupled memory)
    // 17 - set DTCM load mode to r/w
    // 18 - disable ITCM (instruction tightly coupled memory)
    // 19 - set ITCM load mode to r/w
    mrc        p15, #0, r0, cr1, cr0, #0
    ldr        r1, DAT_02004aa8
    bic        r0, r0, r1
    mcr        p15, #0, r0, cr1, cr0, #0

    mov        r0, #0
    // Invalidate instruction cache
    mcr        p15, #0, r0, cr7, cr5, #0
    // Invalidate data cache
    mcr        p15, #0, r0, cr7, cr6, #0
    // Drain write buffer
    mcr        p15, #0, r0, cr7, cr10, #4
    
    ldr        r0, DAT_02004aac
    mcr        p15, #0, r0, cr6, cr0, #0
    
    ldr        r0, DAT_02004ab0
    mcr        p15, #0, r0, cr6, cr1, #0
    
    ldr        r0, DAT_02004ab4
    mcr        p15, #0, r0, cr6, cr2, #0
    
    ldr        r0, DAT_02004ab8
    mcr        p15, #0, r0, cr6, cr3, #0
    
    ldr        r0, DAT_02004abc
    orr        r0, r0, #0x1a
    orr        r0, r0, #1
    mcr        p15, #0, r0, cr6, cr4, #0
    
    ldr        r0, DAT_02004ac0
    mcr        p15, #0, r0, cr6, cr5, #0
    
    ldr        r0, DAT_02004ac4
    mcr        p15, #0, r0, cr6, cr6, #0
    
    ldr        r0, DAT_02004ac8
    mcr        p15, #0, r0, cr6, cr7, #0
    
    mov        r0, #0x20
    mcr        p15, #0, r0, cr9, cr1, #1
    
    ldr        r0, DAT_02004abc
    orr        r0, r0, #0xa
    mcr        p15, #0, r0, cr9, cr1, #0
    
    mov        r0, #0x42
    mcr        p15, #0, r0, cr2, cr0, #1
    
    mov        r0, #0x42
    mcr        p15, #0, r0, cr2, cr0, #0
    
    mov        r0, #2
    mcr        p15, #0, r0, cr3, cr0, #0
    
    ldr        r0, DAT_02004acc
    mcr        p15, #0, r0, cr5, cr0, #3
    
    ldr        r0, DAT_02004ad0
    mcr        p15, #0, r0, cr5, cr0, #2
    
    mrc        p15, #0, r0, cr1, cr0, #0
    ldr        r1, DAT_02004ad4
    orr        r0, r0, r1
    mcr        p15, #0, r0, cr1, cr0, #0
    
    bx         lr
    
DAT_02004aa8:
    .word 0x000f9005
DAT_02004aac:
    .word 0x04000033
DAT_02004ab0:
    .word 0x0200002b
DAT_02004ab4:
    .word 0x027c0023
DAT_02004ab8:
    .word 0x08000035
DAT_02004abc:
    .word 0x023c0000
DAT_02004ac0:
    .word 0x0100002f
DAT_02004ac4:
    .word 0xffff001d
DAT_02004ac8:
    .word 0x027ff017
DAT_02004acc:
    .word 0x05100011
DAT_02004ad0:
    .word 0x15111011
DAT_02004ad4:
    .word 0x0005707d

DAT_02004ad8:
    .word 0x02098FC0
DAT_02004adc:
    .word 0x02098FD8
DAT_02004ae0:
    .word 0x02093100
DAT_02004ae4:
    .word 0x02093100
DAT_02004ae8:
    .word 0x020A24E0
DAT_02004aec:
    .word _COMPRESSED_END_
