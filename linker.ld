ENTRY(_start)

SECTIONS {
    /* Multiboot requires the kernel to be loaded at 1MB or higher */
    . = 1M;

    .boot :
    {
        /* The Multiboot header must be within the first 8192 bytes of the ELF */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        *(.bss .bss.*)
    }
}
