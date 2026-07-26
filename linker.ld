ENTRY(_start)

SECTIONS
{
    . = 0x40080000;

    .text : ALIGN(4096)
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : ALIGN(4096)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4096)
    {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4096)
    {
        _bss_start = .;
        *(.bss.stack)
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    }

    /DISCARD/ :
    {
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
    }
}
