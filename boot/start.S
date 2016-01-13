.text
.global _start

#define arg1 %rdi
# Xen places the address of the start_info struct into the SI register.
#define start_info_addr %rsi

_start:
    mov $STACK_BOTTOM, %rbp
    mov %rbp, %rsp
    mov start_info_addr, start_info
    call main
    hlt    

.bss

start_info:
    .quad 0
