.section .text.boot
.global _start
.type _start, %function

_start:
    ldr x0, =_bss_start
    ldr x1, =_bss_end

.Lclear_bss:
    cmp x0, x1
    b.hs .Lbss_done

    str xzr, [x0], #8
    b .Lclear_bss

.Lbss_done:
    ldr x0, =__stack_top
    mov sp, x0

    ldr x0, =exception_vector_table
    msr VBAR_EL1, x0
    isb

    bl kernel_main

.Lhang:
    wfe
    b .Lhang

.section .bss.stack, "aw", @nobits
.align 12

__stack_bottom:
    .skip 65536

__stack_top:
