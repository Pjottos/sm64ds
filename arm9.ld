ENTRY(_start)

INPUT
(
    start.o
    decompress.o
    init_data_sections.o
    init_cp15.o
)

SECTIONS
{
    .text (0x02004800) : {
        start.o (.text)
        decompress.o (.text)
        init_data_sections.o (.text)
        init_cp15.o (.text)
    }

    .compressed ALIGN(0x1000) : {
        _COMPRESSED_START_ = .;
        _COMPRESSED_END_ = .;
    }
}
