use std::ops::Range;

/// Extract BLZ region with the end of the footer at the specified offset, in place.
/// Returns range of decompressed data.
pub fn extract(buf: &mut [u8], offset: usize) -> Range<usize> {
    assert!(offset < buf.len(), "invalid compressed data offset");
    assert!(
        buf.len() - offset >= 8,
        "compressed data is too short to contain footer"
    );

    let mut compressed_end = offset;
    let flags = u32::from_le_bytes(
        buf[compressed_end - 8..compressed_end - 4]
            .try_into()
            .unwrap(),
    );
    let footer_len = (flags >> 24) as usize;
    let compressed_len = (flags & 0x00ffffff) as usize;
    let dest_offset =
        u32::from_le_bytes(buf[compressed_end - 4..compressed_end].try_into().unwrap()) as usize;

    // eprintln!("Footer length:           {:#010x}", footer_len);
    // eprintln!("Total compressed length: {:#010x}", compressed_len);
    // eprintln!("Destination offset:      {:#010x}", dest_offset);

    assert!(
        footer_len <= buf.len(),
        "footer length is larger than input buffer"
    );
    compressed_end -= footer_len;

    assert!(
        compressed_len <= compressed_end,
        "compressed data length is larger than input buffer (excluding the footer)"
    );
    let compressed_start = offset - compressed_len;

    let mut dest = offset + dest_offset;
    assert!(dest < buf.len(), "invalid destination offset");

    // eprintln!("Compressed data start:   {:#010x}", compressed_start);
    // eprintln!("Compressed data end:     {:#010x}", compressed_end);
    // eprintln!("Decompressed end:        {:#010x}", dest);

    while compressed_end > compressed_start {
        compressed_end -= 1;
        let block_flags = buf[compressed_end];

        for i in 0..8 {
            if block_flags & (0x80 >> i) == 0 {
                compressed_end -= 1;
                dest -= 1;
                buf[dest] = buf[compressed_end];
            } else {
                compressed_end -= 1;
                let a = u32::from(buf[compressed_end]);
                compressed_end -= 1;
                let b = u32::from(buf[compressed_end]);

                let repeat_offset = ((((a << 8) | b) & !0xf000) + 2) as usize;

                let count = (a >> 4) + 3;
                for _ in 0..count {
                    let val = buf[dest + repeat_offset];
                    dest -= 1;
                    buf[dest] = val;
                }
            }

            if compressed_end <= compressed_start {
                break;
            }
        }
    }

    dest..offset + dest_offset
}
