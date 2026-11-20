OUTPUT_ARCH(riscv)
ENTRY(_boot)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 128K
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS
{
    .text : {
        *(.text._boot)
        *(.text)
        *(.text.*)
    } > FLASH

    .rodata : {
        *(.rodata)
        *(.rodata.*)
    } > FLASH

    .data : {
        *(.data)
        *(.data.*)
    } > RAM

    .bss :
    {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    /DISCARD/ : { *(.eh_frame*) *(.comment) }
}
