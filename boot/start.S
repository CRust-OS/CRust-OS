.text
.global _start

_start:
    mov $STACK_TOP, %rbp
    mov %rbp, %rsp
    # Xen places the address of the start_info struct into the SI register.
    mov %rsi, start_info_page
    call prologue
    call poweroff
.bss

.global start_info_page
start_info_page:
    .quad 0
