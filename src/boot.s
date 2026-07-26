.section .text.boot
.global _start
.type _start, %function

_start:
    ldr x0, =__stack_top
    mov sp, x0

    bl kernel_main

1:
    wfe
    b 1b

.section .bss.stack, "aw", @nobits
.align 12

__stack_bottom:
    .skip 65536

__stack_top:
