.global decompress_code

// Params:
//     r0 - End of compressed region
decompress_code:
    cmp        r0, #0
    beq        _02004978
    stmdb      sp!, { r4, r5, r6, r7 }

    // Load (footer_size, data_size) into r1 and decompressed_offset into r2
    ldmdb      r0, { r1, r2 }
    // Set r2 to decompressed_end
    add        r2, r0, r2
    // Set r3 to the end of the compressed data 
    sub        r3, r0, r1, lsr #24
    // Clear footer_size
    bic        r1, r1, #0xff000000
    // Set r1 to the start of the compressed data
    sub        r1, r0, r1
    mov        r4, r2

_020048fc:
    // while compressed_end > compressed_start 
    cmp        r3, r1
    ble        _0200495c
    // compressed_end -= 1
    // block_flags = *compressed_end
    ldrb       r5, [r3, #-1]!

    // for i in 0..8
    mov        r6, #8
_0200490c:
    subs       r6, r6, #1
    blt        _020048fc

    // if block_flags & 0x80 == 0 
    tst        r5, #0x80
    bne        _02004928
    // then
    // compressed_end -= 1
    // decompressed_end -= 1
    // *decompressed_end = *compressed_end
    ldrb       r0, [r3, #-1]!
    strb       r0, [r2, #-1]!
    b          _02004950
_02004928:
    // else
    ldrb       r12, [r3, #-1]!
    ldrb       r7, [r3, #-1]!
    orr        r7, r7, r12, lsl #8
    bic        r7, r7, #0xf000
    add        r7, r7, #2
    add        r12, r12, #0x20
_02004940:
    // do
    ldrb       r0, [r2, r7]
    strb       r0, [r2, #-1]!
    subs       r12, r12, #0x10
    // while r12 >= 0 
    bge        _02004940
_02004950:
    // (see below)
    cmp        r3, r1
    // block_flags <<= 1
    mov        r5, r5, lsl #1
    // if compressed_end <= compressed_start
    // break
    bgt        _0200490c
_0200495c:
    // Align compressed_start to 32 byte cache line
    bic        r3, r1, #0x1f
_02004960:
    // Invalidate instruction cache line
    mcr        p15, #0, r3, cr7, cr5, #1
    // Invalidate data cache line
    mcr        p15, #0, r3, cr7, cr14, #1

    // Advance to next line
    add        r3, #0x20
    cmp        r3, r4
    blt        _02004960

    ldmia      sp!, { r4, r5, r6, r7 }
_02004978:
    bx         lr
