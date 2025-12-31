ENTRY(_start)

MEMORY
{
    RAM : ORIGIN = 0x80000, LENGTH = 512M
}

SECTIONS
{
    .text ORIGIN(RAM) :
    {
        *(.text._start)
        *(.text*)
    } > RAM

    .rodata :
    {
        *(.rodata*)
    } > RAM

    .data :
    {
        _data_start = .;
        *(.data*)
        _data_end = .;
    } > RAM

    .bss :
    {
        _bss_start = .;
        *(.bss*)
        *(COMMON)
        _bss_end = .;
    } > RAM

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.comment)
    }
}
