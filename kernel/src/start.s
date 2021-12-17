.extern LD_STACK_PTR

.section ".text.boot"

.global _start

_start:
    //stack is setup by loader
    // clear bss
    ldr     x1, =__bss_start
    ldr     w2, =__bss_size
1:  cbz     w2, 2f
    str     xzr, [x1], #8
    sub     w2, w2, #1
    cbnz    w2, 1b

    // jump to main, should not return
2:  bl      rmain
3:  wfe
    b       3b
