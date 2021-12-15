
SECTIONS
{
    . = 0x80000;
    .text : {
          KEEP(*(.text.boot))
          *(.text .text.*)
    }

    .rodata : {
            *(.rodata .rodata.*)
    }

    .data : {
          *(.data .data.*)
    }

    .bss : {
        . = ALIGN(16);
        __bss_start = .;
         *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(0x10);
    . += 0x1000;
    LD_STACK_PTR = .;

   /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}

__bss_size = (__bss_end - __bss_start)>>3;
