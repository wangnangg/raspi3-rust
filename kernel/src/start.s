.extern LD_STACK_PTR

.section ".text.boot"

.global _start

_start:
    //we must be in el2

    // clear bss
    // adr can load label address in a position independent manner
    adr     x1, __bss_start
    ldr     w2, =__bss_size
1:  cbz     w2, switch_to_el1
    str     xzr, [x1], #8
    sub     w2, w2, #1
    cbnz    w2, 1b

switch_to_el1:
    //set stack pointer, adrp + add allows farther label address than just adr
    adrp x1, __exception_stack
    add x1, x1, #:lo12:__exception_stack
    msr SP_EL1, x1

    adrp x1, __main_stack
    add x1, x1, #:lo12:__main_stack
    msr SP_EL0, x1

    //enalbe AArch64 in EL1, do not trap anything to EL2 (disable hypervisor)
    mov     x0, #(1 << 31)      // Enable AArch64 for EL1
    orr     x0, x0, #(1 << 1)   // RES1 on A-53
    msr     HCR_EL2, x0

    // Set SCTLR to known state (set RES1 only: 11, 20, 22, 23, 28, 29) (A53: 4.3.30)
    mov     x2, #0x0800
    movk    x2, #0x30d0, lsl #16
    msr     SCTLR_EL1, x2

    // set up exception handlers
    // load `el1_exception_table` addr into VBAR_EL1
    adr x1, el1_exception_table
    msr VBAR_EL1, x1

    // change execution level to EL1 (ref: C5.2.19)
    // M[3:0] == 0b0100, EL1t, use SP_EL0
    // M[4] == 0, return to AArch64
    // DAIF == 0b1111, watchpoint/breakpoint/step, SError, IRQ, FIQ exceptions masked
    // other flags are set to zero
    mov     x2, #0x3c4
    msr     SPSR_EL2, x2

    // Return to EL1 at `el1_start`.
    adr x1, el1_start
    msr ELR_EL2, x1
    eret

el1_start:
    // jump to main, should not return
2:  bl      rmain
3:  wfe
    b       3b

context_save:
    //put syndrome to x1
    mrs x1, ESR_EL1
    //push caller-saved registers: x0..x18, x30 (lr)
    stp x0, x1, [SP, #-16]!
    stp x2, x3, [SP, #-16]!
    stp x4, x5, [SP, #-16]!
    stp x6, x7, [SP, #-16]!
    stp x8, x9, [SP, #-16]!
    stp x10, x11, [SP, #-16]!
    stp x12, x13, [SP, #-16]!
    stp x14, x15, [SP, #-16]!
    stp x16, x17, [SP, #-16]!
    stp x18, x30, [SP, #-16]!
    bl handle_exception
context_restore:
    ldp x18, x30, [SP], #16
    ldp x16, x17, [SP], #16
    ldp x14, x15, [SP], #16
    ldp x12, x13, [SP], #16
    ldp x10, x11, [SP], #16
    ldp x8, x9, [SP], #16
    ldp x6, x7, [SP], #16
    ldp x4, x5, [SP], #16
    ldp x2, x3, [SP], #16
    ldp x0, x1, [SP], #16
    ret

.macro handler source, kind
.align 7
stp     lr, x0, [SP, #-16]!
mov     x0, \source
movk    x0, \kind, LSL #16
bl      context_save
ldp     lr, x0, [SP], #16
eret
.endm

//align to table size : 16 entreis * 128 bytes = 2048
.align 11
el1_exception_table:

//same level with SP0
//synchronous
handler 0, 0
//irq
handler 0, 1
//fiq
handler 0, 2
//serror
handler 0, 3

//same level with SPx
//synchronous
handler 1, 0
//irq
handler 1, 1
//fiq
handler 1, 2
//serror
handler 1, 3

//lower level with AArch64
handler 2, 0
//irq
handler 2, 1
//fiq
handler 2, 2
//serror
handler 2, 3

//lower level with AArch32
handler 3, 0
//irq
handler 3, 1
//fiq
handler 3, 2
//serror
handler 3, 3
