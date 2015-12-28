.text
.global _start

_start:
    mov $STACK_BOTTOM, %rbp
    mov %rbp, %rsp
    movq %rsi, %rdi # Xen places the address of the start_info struct into RSI register. Move it to RDI register for main
    call main
    hlt    
