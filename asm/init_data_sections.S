.global DAT_02004ad8

.global init_data_sections

init_data_sections:
    ldr        r0, PTR_DAT_020049e8
    // Load data_meta_start
    ldr        r1, [r0, #0]
    // Load data_meta_end
    ldr        r2, [r0, #4]
    // Load data
    ldr        r3, [r0, #8]
_0200498c:
    // while data_meta_start != data_meta_end
    cmp        r1, r2
    beq        _020049cc

    // Load target_start
    ldr        r4, [r1], #4
    // Load target_size
    ldr        r5, [r1], #4
    // target_end = target_start + target_size 
    add        r6, r4, r5
_020049a0:
    // while target_start < target_end
    // BUG: Uses the `mi` condition instead of `cc`
    // This means that if target_start >= 0x80000000 and target_end < target_start, the
    // loop will keep going past the end. In practice this will never
    // happen though, because that is not a valid address.
    cmp        r4, r6
    // *target_start = *data
    // target_start += 4
    // data += 4
    ldrmi      r7, [r3], #4
    strmi      r7, [r4], #4
    bmi        _020049a0

    // Load pad_size
    ldr        r5, [r1], #4
    // target_end = target_start + pad_size
    add        r6, r4, r5

    // Pad with 0
    mov        r7, #0
_020049bc:
    cmp        r4, r6
    strcc      r7, [r4], #4
    bcc        _020049bc
    
    // Fill data sections until target_start & 3 != target_end & 3
    beq        _0200498c
    
_020049cc:
    // Load bss_start
    ldr        r4, [r0, #0xc]
    // Load bss_end
    ldr        r6, [r0, #0x10]
    
    // Initialize bss
    mov        r7, #0
_020049d8:
    cmp        r4, r6
    strcc      r7, [r4], #4
    bcc        _020049d8
    
    // ???
    b          _020049ec
PTR_DAT_020049e8:
    .word DAT_02004ad8
_020049ec:
    bx         lr
