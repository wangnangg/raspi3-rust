/* changing this file requires clean bulid */

SECTIONS
{
    . = 0x0;
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

    . += 1024 * 1024;
    . = ALIGN(16);
    __main_stack = .;

    . += 1024 * 1024;
    . = ALIGN(16);
    __exception_stack = .;

   /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}

__bss_size = (__bss_end - __bss_start)>>3;
