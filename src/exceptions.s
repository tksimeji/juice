.section .text.exceptions, "ax"
.balign 2048

.macro vector_entry
    b .Lunexpected_exception
    .space 124
.endm

.global exception_vector_table
.type exception_vector_table, %object

exception_vector_table:
    vector_entry
    vector_entry
    vector_entry
    vector_entry

    vector_entry
    vector_entry
    vector_entry
    vector_entry

    vector_entry
    vector_entry
    vector_entry
    vector_entry

    vector_entry
    vector_entry
    vector_entry
    vector_entry

.Lunexpected_exception:
    bl exception_dispatch

.Lexception_hang:
    wfe
    b .Lexception_hang
